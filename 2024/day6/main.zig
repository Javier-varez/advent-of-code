const std = @import("std");

const DimError = error{DimError};

const Map = struct {
    width: usize,
    height: usize,
    stride: usize,

    map: []u8,

    fn init(data: []u8) DimError!Map {
        const total_length = data.len;

        const width = for (data, 0..) |byte, idx| {
            if (byte == '\n') {
                break idx;
            }
        } else {
            return DimError.DimError;
        };

        const stride = width + 1;

        return Map{ .width = @intCast(width), .height = @intCast(total_length / stride), .stride = @intCast(stride), .map = data };
    }

    fn findInitialLocation(self: *const Map) ?Location {
        for (self.map, 0..) |byte, idx| {
            if (byte == '^') {
                const r = idx / self.stride;
                const c = idx - r * self.stride;
                return Location{ .r = @intCast(r), .c = @intCast(c) };
            }
        }
        return null;
    }

    fn at(self: *const Map, l: Location) u8 {
        if ((l.r >= self.height or l.r < 0) or
            (l.c >= self.width or l.c < 0))
        {
            return '-';
        }
        const r: usize = @intCast(l.r);
        const c: usize = @intCast(l.c);
        return self.map[r * self.stride + c];
    }

    fn atRef(self: *Map, l: Location) *u8 {
        std.debug.assert(l.isValid(self));
        const r: usize = @intCast(l.r);
        const c: usize = @intCast(l.c);
        return &self.map[r * self.stride + c];
    }
};

const Location = struct {
    r: isize,
    c: isize,

    fn isValid(self: *const Location, map: *const Map) bool {
        return self.r >= 0 and self.c >= 0 and self.r < map.height and self.c < map.width;
    }
};

const Direction = enum(u2) {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,

    fn rotate(self: Direction) Direction {
        switch (self) {
            .Up => {
                return .Right;
            },
            .Down => {
                return .Left;
            },
            .Left => {
                return .Up;
            },
            .Right => {
                return .Down;
            },
        }
    }
};

const Guard = struct {
    location: Location,
    direction: Direction,

    fn move(self: *Guard, map: *const Map) bool {
        const nl = self.nextLocation();
        if (!nl.isValid(map)) {
            return false;
        }

        if (map.at(nl) == '#') {
            self.direction = self.direction.rotate();
            return self.move(map);
        }

        self.location = nl;
        return true;
    }

    fn nextLocation(self: *const Guard) Location {
        const deltaC = [4]isize{ 0, 0, -1, 1 };
        const deltaR = [4]isize{ -1, 1, 0, 0 };

        const direction = @intFromEnum(self.direction);
        return Location{ .c = deltaC[direction] + self.location.c, .r = deltaR[direction] + self.location.r };
    }
};

fn part1(allocator: std.mem.Allocator, map: *const Map) !void {
    var guard = Guard{ .location = map.findInitialLocation().?, .direction = .Up };
    var visitedLocations = std.AutoHashMap(Location, void).init(allocator);
    defer visitedLocations.deinit();

    while (guard.move(map)) {
        _ = try visitedLocations.getOrPut(guard.location);
    }
    std.debug.print("Visited positions: {}\n", .{visitedLocations.count()});
}

fn testLoop(allocator: std.mem.Allocator, map: *const Map) !bool {
    const initialLoc = map.findInitialLocation().?;
    var guard = Guard{ .location = initialLoc, .direction = .Up };

    var visitedLocations = std.AutoHashMap(Location, std.AutoHashMap(Direction, void)).init(allocator);
    defer {
        var iter = visitedLocations.valueIterator();
        while (iter.next()) |v| {
            v.deinit();
        }
        visitedLocations.deinit();
    }

    while (guard.move(map)) {
        var entry = try visitedLocations.getOrPut(guard.location);
        if (!entry.found_existing) {
            entry.value_ptr.* = std.AutoHashMap(Direction, void).init(allocator);
        }

        if (entry.value_ptr.contains(guard.direction)) {
            return true;
        }

        try entry.value_ptr.put(guard.direction, {});
    }

    return false;
}

fn part2(allocator: std.mem.Allocator, map: *Map) !void {
    var count: usize = 0;
    for (0..map.width) |c| {
        for (0..map.height) |r| {
            const loc = Location{ .r = @intCast(r), .c = @intCast(c) };
            if (map.at(loc) == '.') {
                map.atRef(loc).* = '#';
                if (try testLoop(allocator, map)) {
                    count += 1;
                }
                map.atRef(loc).* = '.';
            }
        }
    }
    std.debug.print("Loops {}\n", .{count});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        const result = gpa.deinit();
        if (result == .leak) {
            std.log.err("Memory leak\n", .{});
        }
    }

    const input = try std.fs.cwd().readFileAlloc(allocator, "realinput.txt", std.math.maxInt(usize));
    defer allocator.free(input);
    var map = try Map.init(input);
    try part1(allocator, &map);
    try part2(allocator, &map);
}
