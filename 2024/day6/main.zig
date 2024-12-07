const std = @import("std");

const DimError = error{DimError};

const Map = struct {
    width: usize,
    height: usize,
    stride: usize,

    map: []const u8,

    fn init(data: []const u8) DimError!Map {
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

    fn rotate(self: *Direction) void {
        switch (self.*) {
            .Up => {
                self.* = .Right;
            },
            .Down => {
                self.* = .Left;
            },
            .Left => {
                self.* = .Up;
            },
            .Right => {
                self.* = .Down;
            },
        }
    }
};

const Guard = struct {
    location: Location,
    direction: Direction,

    fn move(self: *Guard, map: *const Map) bool {
        const deltaC = [4]isize{ 0, 0, -1, 1 };
        const deltaR = [4]isize{ -1, 1, 0, 0 };

        const direction = @intFromEnum(self.direction);
        const nextLocation = Location{ .c = deltaC[direction] + self.location.c, .r = deltaR[direction] + self.location.r };
        if (!nextLocation.isValid(map)) {
            return false;
        }

        if (map.at(nextLocation) == '#') {
            self.direction.rotate();
        } else {
            self.location = nextLocation;
        }

        return true;
    }
};

fn part1(allocator: std.mem.Allocator, map: *const Map) !void {
    var guard = Guard{ .location = map.findInitialLocation().?, .direction = .Up };
    var visitedLocations = std.AutoHashMap(Location, void).init(allocator);

    while (guard.move(map)) {
        _ = try visitedLocations.getOrPut(guard.location);
    }
    std.debug.print("Visited positions: {}\n", .{visitedLocations.count()});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const input = try std.fs.cwd().readFileAlloc(allocator, "realinput.txt", std.math.maxInt(usize));
    const map = try Map.init(input);
    try part1(allocator, &map);
}
