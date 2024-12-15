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

    try part2(allocator, data);
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
            if (byte == '@') {
                try mapData.append(byte);
                try mapData.append('.');
            } else if (byte == 'O') {
                try mapData.append('[');
                try mapData.append(']');
            } else if (byte == '\n') {
                try mapData.append('\n');
            } else {
                try mapData.append(byte);
                try mapData.append(byte);
            }
        }

        return Map{ .width = width * 2, .height = height, .stride = width * 2 + 1, .data = mapData };
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
                if (l.r > 0) {
                    return Location{ .r = l.r - 1, .c = l.c };
                } else {
                    return null;
                }
            },
            'v' => {
                if (l.r < self.height - 1) {
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

fn canPushBlock(map: *Map, l: Location, dir: u8) bool {
    const proposed = map.move(dir, l);
    if (proposed == null) {
        std.log.debug("invalid proposed after {},{} dir {c}", .{ l.r, l.c, dir });
        return false;
    }

    const proposedBlock = map.at(proposed.?);

    if (proposedBlock.* == '.') {
        return true;
    } else if (proposedBlock.* == '[' and (dir == '^' or dir == 'v')) {
        var canPush = true;
        const sibling = map.move('>', proposed.?).?;
        canPush = canPush and canPushBlock(map, proposed.?, dir);
        canPush = canPush and canPushBlock(map, sibling, dir);
        return canPush;
    } else if (proposedBlock.* == ']' and (dir == '^' or dir == 'v')) {
        var canPush = true;
        const sibling = map.move('<', proposed.?).?;
        canPush = canPush and canPushBlock(map, proposed.?, dir);
        canPush = canPush and canPushBlock(map, sibling, dir);
        return canPush;
    } else if (proposedBlock.* == ']' or proposedBlock.* == '[') {
        return canPushBlock(map, proposed.?, dir);
    }
    return false;
}

fn pushBlock(map: *Map, l: Location, dir: u8) Location {
    const proposed = map.move(dir, l).?;
    const proposedBlock = map.at(proposed);

    if (proposedBlock.* == '.') {} else if (proposedBlock.* == '[' and (dir == '^' or dir == 'v')) {
        const sibling = map.move('>', proposed).?;
        _ = pushBlock(map, sibling, dir);
        _ = pushBlock(map, proposed, dir);
    } else if (proposedBlock.* == ']' and (dir == '^' or dir == 'v')) {
        const sibling = map.move('<', proposed).?;
        _ = pushBlock(map, sibling, dir);
        _ = pushBlock(map, proposed, dir);
    } else if (proposedBlock.* == '[' or proposedBlock.* == ']') {
        _ = pushBlock(map, proposed, dir);
    } else {
        @panic("pushBlock failed");
    }
    const curBlock = map.at(l);
    proposedBlock.* = curBlock.*;
    curBlock.* = '.';
    return proposed;
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

fn part2(allocator: std.mem.Allocator, data: []u8) !void {
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

        if (canPushBlock(&input.map, robot, mov)) {
            // std.debug.print("move approved {c}\n", .{mov});
            robot = pushBlock(&input.map, robot, mov);
        }
        // std.debug.print("after {c}\n", .{mov});
        // print(&input.map);
    }

    var result: usize = 0;
    for (0..input.map.height) |r| {
        for (0..input.map.width) |c| {
            const l = Location{ .r = r, .c = c };
            const block = input.map.at(l).*;
            if (block == '[') {
                result += 100 * r + c;
            }
        }
    }
    std.debug.print("part 2 = {}\n", .{result});
}
