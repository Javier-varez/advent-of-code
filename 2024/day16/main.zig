const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const result = gpa.deinit();
        if (result == .leak) {
            std.log.err("Memory leak", .{});
        }
    }
    const allocator = gpa.allocator();
    const data = try std.fs.cwd().readFileAlloc(allocator, "realinput.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    try part1and2(allocator, data);
}

const Location = struct {
    r: usize,
    c: usize,
};

const Map = struct {
    width: usize,
    height: usize,
    stride: usize,
    data: []const u8,

    pub fn init(data: []const u8) Map {
        const width: usize = for (data, 0..) |byte, idx| {
            if (byte == '\n') {
                break idx;
            }
        } else @panic("Did not find more than one line in the map");

        const stride: usize = width + 1;
        const height = data.len / stride;

        return Map{ .width = width, .height = height, .stride = stride, .data = data };
    }

    pub fn at(self: *const Map, l: Location) u8 {
        return self.data[l.r * self.stride + l.c];
    }

    pub fn startNode(self: *const Map) Location {
        for (self.data, 0..) |byte, idx| {
            if (byte == 'S') {
                const r = idx / self.stride;
                const c = idx % self.stride;
                return Location{ .r = r, .c = c };
            }
        }
        @panic("Did not find start node");
    }

    pub fn endNode(self: *const Map) Location {
        for (self.data, 0..) |byte, idx| {
            if (byte == 'E') {
                const r = idx / self.stride;
                const c = idx % self.stride;
                return Location{ .r = r, .c = c };
            }
        }
        @panic("Did not find end node");
    }

    pub fn move(self: *const Map, dir: Dir, l: Location) ?Location {
        if (dir.matches(UP)) {
            return self.up(l);
        }
        if (dir.matches(DOWN)) {
            return self.down(l);
        }
        if (dir.matches(LEFT)) {
            return self.left(l);
        }
        if (dir.matches(RIGHT)) {
            return self.right(l);
        }
        unreachable;
    }

    pub fn up(_: *const Map, l: Location) ?Location {
        if (l.r == 0) {
            return null;
        }
        return Location{ .r = l.r - 1, .c = l.c };
    }

    pub fn down(self: *const Map, l: Location) ?Location {
        if (l.r >= self.height - 1) {
            return null;
        }
        return Location{ .r = l.r + 1, .c = l.c };
    }

    pub fn left(_: *const Map, l: Location) ?Location {
        if (l.c == 0) {
            return null;
        }
        return Location{ .r = l.r, .c = l.c - 1 };
    }

    pub fn right(self: *const Map, l: Location) ?Location {
        if (l.c >= self.width - 1) {
            return null;
        }
        return Location{ .r = l.r, .c = l.c + 1 };
    }
};

const Dir = struct {
    r: isize,
    c: isize,

    pub fn matches(self: Dir, other: Dir) bool {
        return self.r == other.r and self.c == other.c;
    }

    pub fn rotateClockwise(self: Dir) Dir {
        if (self.matches(UP)) return RIGHT;
        if (self.matches(RIGHT)) return DOWN;
        if (self.matches(DOWN)) return LEFT;
        if (self.matches(LEFT)) return UP;
        unreachable;
    }

    pub fn rotateCounterClockwise(self: Dir) Dir {
        if (self.matches(UP)) return LEFT;
        if (self.matches(LEFT)) return DOWN;
        if (self.matches(DOWN)) return RIGHT;
        if (self.matches(RIGHT)) return UP;
        unreachable;
    }
};

const UP: Dir = Dir{ .r = -1, .c = 0 };
const DOWN: Dir = Dir{ .r = 1, .c = 0 };
const LEFT: Dir = Dir{ .r = 0, .c = -1 };
const RIGHT: Dir = Dir{ .r = 0, .c = 1 };

const LocAndDir = struct { l: Location, dir: Dir };
const LocAndDirWithCost = struct {
    l: Location,
    dir: Dir,
    minCost: usize,

    fn locAndDir(self: LocAndDirWithCost) LocAndDir {
        return LocAndDir{ .l = self.l, .dir = self.dir };
    }
};

fn lessThan(context: void, l: LocAndDirWithCost, r: LocAndDirWithCost) std.math.Order {
    _ = context;
    return std.math.order(l.minCost, r.minCost);
}

const PriorityQueue = std.PriorityQueue(LocAndDirWithCost, void, lessThan);

const AlgoState = struct {
    unvisitedNodes: PriorityQueue,
    costPerLocAndDir: std.AutoHashMap(LocAndDir, usize),
    backtrace: std.AutoHashMap(LocAndDir, std.ArrayList(LocAndDir)),

    fn init(allocator: std.mem.Allocator) AlgoState {
        const unvisitedNodes = PriorityQueue.init(allocator, {});
        const costPerLocAndDir = std.AutoHashMap(LocAndDir, usize).init(allocator);
        const backtrace = std.AutoHashMap(LocAndDir, std.ArrayList(LocAndDir)).init(allocator);
        return AlgoState{ .unvisitedNodes = unvisitedNodes, .costPerLocAndDir = costPerLocAndDir, .backtrace = backtrace };
    }

    fn deinit(self: *AlgoState) void {
        self.unvisitedNodes.deinit();
        self.costPerLocAndDir.deinit();
        var iter = self.backtrace.valueIterator();
        while (iter.next()) |elem| {
            elem.deinit();
        }
        self.backtrace.deinit();
    }
};

fn drawAndCountSafeLocations(allocator: std.mem.Allocator, map: *const Map, state: *const AlgoState, endEntry: LocAndDirWithCost) !usize {
    var visited = std.AutoHashMap(LocAndDir, void).init(allocator);
    defer visited.deinit();

    var queue = std.ArrayList(LocAndDir).init(allocator);
    defer queue.deinit();

    try queue.append(endEntry.locAndDir());

    while (queue.popOrNull()) |cur| {
        if (visited.contains(cur)) {
            continue;
        }

        try visited.put(cur, {});

        if (state.backtrace.get(cur)) |prevEntries| {
            for (prevEntries.items) |prev| {
                try queue.append(prev);
            }
        }
    }

    var visitedOnlyLocations = std.AutoHashMap(Location, void).init(allocator);
    defer visitedOnlyLocations.deinit();

    var iter = visited.keyIterator();
    while (iter.next()) |locAndDir| {
        try visitedOnlyLocations.put(locAndDir.l, {});
    }

    for (0..map.height) |r| {
        for (0..map.width) |c| {
            if (visitedOnlyLocations.contains(Location{ .r = r, .c = c })) {
                std.debug.print("O", .{});
            } else {
                std.debug.print("{c}", .{map.at(Location{ .r = r, .c = c })});
            }
        }
        std.debug.print("\n", .{});
    }
    return visitedOnlyLocations.count();
}

fn min(maybeCurCost: ?LocAndDirWithCost, newCost: LocAndDirWithCost) LocAndDirWithCost {
    if (maybeCurCost == null) {
        return newCost;
    }

    if (maybeCurCost.?.minCost > newCost.minCost) {
        return newCost;
    }
    return maybeCurCost.?;
}

fn part1and2(allocator: std.mem.Allocator, data: []const u8) !void {
    const map = Map.init(data);
    var state = AlgoState.init(allocator);
    defer state.deinit();

    // initialize the queue with the start node
    try state.unvisitedNodes.add(LocAndDirWithCost{
        .l = map.startNode(),
        .dir = RIGHT,
        .minCost = 0,
    });

    while (state.unvisitedNodes.removeOrNull()) |curNode| {
        if (state.costPerLocAndDir.get(curNode.locAndDir())) |costEntry| {
            if (curNode.minCost > costEntry) {
                // This is a higher-cost path
                continue;
            }
        }

        // Try to move in the same direction first
        var directions = [3]LocAndDirWithCost{ undefined, undefined, undefined };
        var locs: usize = 0;
        if (map.move(curNode.dir, curNode.l)) |newLoc| {
            directions[locs] = LocAndDirWithCost{ .l = newLoc, .dir = curNode.dir, .minCost = curNode.minCost + 1 };
            locs += 1;
        }
        if (map.move(curNode.dir.rotateClockwise(), curNode.l)) |newLoc| {
            directions[locs] = LocAndDirWithCost{ .l = newLoc, .dir = curNode.dir.rotateClockwise(), .minCost = curNode.minCost + 1001 };
            locs += 1;
        }
        if (map.move(curNode.dir.rotateCounterClockwise(), curNode.l)) |newLoc| {
            directions[locs] = LocAndDirWithCost{ .l = newLoc, .dir = curNode.dir.rotateCounterClockwise(), .minCost = curNode.minCost + 1001 };
            locs += 1;
        }

        for (directions[0..locs]) |newNode| {
            if (map.at(newNode.l) == '#') {
                // Not a valid location
                continue;
            }

            const costNode = try state.costPerLocAndDir.getOrPut(newNode.locAndDir());
            if (costNode.found_existing) {
                if (costNode.value_ptr.* < newNode.minCost) {
                    // Not good, too costly
                    continue;
                }

                if (costNode.value_ptr.* > newNode.minCost) {
                    // We found a better way to get to this node
                    try state.backtrace.getPtr(newNode.locAndDir()).?.resize(0);
                }
                try state.backtrace.getPtr(newNode.locAndDir()).?.append(curNode.locAndDir());
            } else {
                var bt = std.ArrayList(LocAndDir).init(allocator);
                try bt.append(curNode.locAndDir());
                try state.backtrace.put(newNode.locAndDir(), bt);
            }

            costNode.value_ptr.* = newNode.minCost;
            try state.unvisitedNodes.add(newNode);
        }
    }

    var maybeMinCost: ?LocAndDirWithCost = null;
    for (&[4]Dir{ DOWN, UP, LEFT, RIGHT }) |dir| {
        const locAndDir = LocAndDir{ .l = map.endNode(), .dir = dir };
        if (state.costPerLocAndDir.get(locAndDir)) |costForDir| {
            const new = min(maybeMinCost, LocAndDirWithCost{ .l = locAndDir.l, .dir = locAndDir.dir, .minCost = costForDir });
            maybeMinCost = new;
        }
    }

    if (maybeMinCost) |minCost| {
        const count = try drawAndCountSafeLocations(allocator, &map, &state, minCost);
        std.debug.print("Min cost is {}, visited nodes {}\n", .{ minCost.minCost, count });
    } else {
        std.log.err("Did not find any solution", .{});
    }
}
