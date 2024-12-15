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

    const data = try std.fs.cwd().readFileAlloc(allocator, "input.txt", std.math.maxInt(usize));
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
    data: std.ArrayList(u8),

    fn init(allocator: std.mem.Allocator, data: []const u8) !Map {
        const width = (for (data, 0..) |byte, idx| {
            if (byte == '\n') {
                break idx;
            }
        } else null).?;

        const stride: usize = width + 1;
        const height = (data.len + stride - 1) / stride;

        var mapData = std.ArrayList(u8).init(allocator);
        errdefer mapData.deinit();
        for (data) |byte| {
            try mapData.append(byte);
        }

        std.log.debug("map width {}, height {}, stride {}, len {}", .{ width, height, stride, data.len });

        return Map{ .width = width, .height = height, .stride = stride, .data = mapData };
    }

    fn deinit(self: *Map) void {
        self.data.deinit();
    }

    fn at(self: *Map, l: Location) *u8 {
        return &self.data.items[l.r * self.stride + l.c];
    }

    fn robotPos(self: *Map) ?Location {
        for (0..self.height) |r| {
            for (0..self.width) |c| {
                const l = Location{ .r = r, .c = c };
                if (self.at(l).* == '@') {
                    return l;
                }
            }
        }
        return null;
    }

    fn move(self: *const Map, dir: u8, l: Location) ?Location {
        switch (dir) {
            '<' => {
                if (l.c > 0) {
                    return Location{ .r = l.r, .c = l.c - 1 };
                } else {
                    return null;
                }
            },
            '>' => {
                if (l.c < self.width - 1) {
                    return Location{ .r = l.r, .c = l.c + 1 };
                } else {
                    return null;
                }
            },
            '^' => {
                if (l.c > 0) {
                    return Location{ .r = l.r - 1, .c = l.c };
                } else {
                    return null;
                }
            },
            'v' => {
                if (l.c < self.height - 1) {
                    return Location{ .r = l.r + 1, .c = l.c };
                } else {
                    return null;
                }
            },
            else => {
                @panic("unknown movement!");
            },
        }
    }
};

const Input = struct {
    map: Map,
    movements: []const u8,
    robot: Location,

    fn init(allocator: std.mem.Allocator, data: []const u8) !Input {
        var iter = std.mem.splitSequence(u8, data, &[2]u8{ '\n', '\n' });
        var map = try Map.init(allocator, iter.next().?);

        const movs = iter.next().?;
        std.log.debug("movements:{s}", .{movs});

        const robot = map.robotPos().?;
        std.log.debug("robot: {}, {}", .{ robot.r, robot.c });

        map.at(robot).* = '.';

        return Input{ .map = map, .movements = movs, .robot = robot };
    }

    fn deinit(self: *Input) void {
        self.map.deinit();
    }
};

fn pushBlock(map: *Map, l: Location, dir: u8) ?Location {
    const proposed = map.move(dir, l);
    if (proposed == null) {
        return null;
    }

    const proposedBlock = map.at(proposed.?);

    if (proposedBlock.* == '.') {
        const curBlock = map.at(l);
        proposedBlock.* = curBlock.*;
        curBlock.* = '.';
        return proposed;
    } else if (proposedBlock.* == 'O') {
        const result = pushBlock(map, proposed.?, dir);
        if (result != null) {
            const curBlock = map.at(l);
            proposedBlock.* = curBlock.*;
            curBlock.* = '.';
            return proposed;
        }
    }
    return null;
}

fn print(map: *Map) void {
    for (0..map.height) |r| {
        for (0..map.width) |c| {
            const l = Location{ .r = r, .c = c };
            const block = map.at(l).*;
            std.debug.print("{c}", .{block});
        }
        std.debug.print("\n", .{});
    }
}

fn part1(allocator: std.mem.Allocator, data: []u8) !void {
    var input = try Input.init(allocator, data);
    defer input.deinit();

    var robot = input.robot;
    input.map.at(robot).* = 'r';
    print(&input.map);

    for (input.movements) |mov| {
        if (mov == '\n') {
            continue;
        }

        std.debug.print("Running instr {c}\n", .{mov});

        if (pushBlock(&input.map, robot, mov)) |l| {
            robot = l;
        }
        print(&input.map);
    }

    var result: usize = 0;
    for (0..input.map.height) |r| {
        for (0..input.map.width) |c| {
            const l = Location{ .r = r, .c = c };
            const block = input.map.at(l).*;
            if (block == 'O') {
                result += 100 * r + c;
            }
        }
    }
    std.debug.print("part 1 = {}\n", .{result});
}
