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

    try part1(allocator, data);
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
const Elem = struct { l: Location, dir: Dir, minCost: usize };

fn lessThan(context: void, l: Elem, r: Elem) std.math.Order {
    _ = context;
    return std.math.order(l.minCost, r.minCost);
}

const PriorityQueue = std.PriorityQueue(Elem, void, lessThan);

const AlgoState = struct {
    unvisitedNodes: PriorityQueue,
    visitedNodes: std.AutoHashMap(Location, void),

    fn init(allocator: std.mem.Allocator) AlgoState {
        const unvisitedNodes = PriorityQueue.init(allocator, {});

        const visitedNodes = std.AutoHashMap(Location, void).init(allocator);
        return AlgoState{ .unvisitedNodes = unvisitedNodes, .visitedNodes = visitedNodes };
    }
    fn deinit(self: *AlgoState) void {
        self.unvisitedNodes.deinit();
        self.visitedNodes.deinit();
    }
};

fn handleDirection(map: *const Map, state: *AlgoState, dir: Dir, loc: Location, newCost: usize) !void {
    const maybeNewLoc = map.move(dir, loc);
    if (maybeNewLoc == null) {
        return;
    }

    const newLoc = maybeNewLoc.?;
    if (map.at(newLoc) == '#') {
        return;
    }

    if (state.visitedNodes.contains(newLoc)) {
        // Already visited, no need to re-evaluate min cost
        return;
    }

    for (state.unvisitedNodes.items, 0..) |*elem, idx| {
        if (elem.l.r == newLoc.r and elem.l.c == newLoc.c) {
            if (elem.minCost > newCost) {
                // Remove the element and insert it again
                _ = state.unvisitedNodes.removeIndex(idx);
            } else {
                // No need to mutate anything
                return;
            }
        }
    }

    try state.unvisitedNodes.add(Elem{ .l = newLoc, .dir = dir, .minCost = newCost });
}

fn part1(allocator: std.mem.Allocator, data: []const u8) !void {
    const map = Map.init(data);
    var state = AlgoState.init(allocator);
    defer state.deinit();

    // initialize the queue with the start node
    try state.unvisitedNodes.add(Elem{ .l = map.startNode(), .dir = RIGHT, .minCost = 0 });

    const maybeMinCost = while (state.unvisitedNodes.removeOrNull()) |curNode| {
        // The current node is now visited
        try state.visitedNodes.put(curNode.l, {});

        if (map.at(curNode.l) == 'E') {
            break curNode.minCost;
        }

        // Try to move in the same direction first
        try handleDirection(&map, &state, curNode.dir, curNode.l, curNode.minCost + 1);
        try handleDirection(&map, &state, curNode.dir.rotateClockwise(), curNode.l, curNode.minCost + 1001);
        try handleDirection(&map, &state, curNode.dir.rotateCounterClockwise(), curNode.l, curNode.minCost + 1001);
    } else null;

    if (maybeMinCost) |minCost| {
        std.debug.print("Min cost is {}", .{minCost});
    } else {
        std.log.err("Did not find any solution", .{});
    }
}
