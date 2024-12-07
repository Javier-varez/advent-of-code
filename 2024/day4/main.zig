const std = @import("std");

const Map = struct {
    width: isize,
    height: isize,
    stride: isize,

    map: []const u8,

    fn new(data: []const u8) DimError!Map {
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

    fn checkSequence(self: *const Map, row: isize, col: isize, d_r: isize, d_c: isize, expected: []const u8) usize {
        var r = row;
        var c = col;

        for (expected) |expectedChar| {
            if (expectedChar != self.at(r, c)) {
                return 0;
            }

            r += d_r;
            c += d_c;
        }

        return 1;
    }

    fn at(self: *const Map, r: isize, c: isize) u8 {
        if ((r >= self.height or r < 0) or
            (c >= self.width or c < 0))
        {
            return '-';
        }
        return self.map[@intCast(r * self.stride + c)];
    }
};

const DimError = error{DimError};

fn solvePart1(data: []const u8) !void {
    const map = try Map.new(data);

    var total: usize = 0;

    var r: isize = 0;
    while (r < map.height) : (r += 1) {
        var c: isize = 0;
        while (c < map.width) : (c += 1) {
            var d_r: isize = -1;
            while (d_r <= 1) : (d_r += 1) {
                var d_c: isize = -1;
                while (d_c <= 1) : (d_c += 1) {
                    if (d_r == 0 and d_c == 0) {
                        continue;
                    }
                    total += map.checkSequence(r, c, d_r, d_c, "XMAS");
                }
            }
        }
    }

    std.debug.print("Part 1 total = {}\n", .{total});
}

fn isMOrS(map: *const Map, r: isize, c: isize) bool {
    const v = map.at(r, c);
    return v == 'M' or v == 'S';
}

fn areEqual(map: *const Map, r: isize, c: isize, r2: isize, c2: isize) bool {
    const v = map.at(r, c);
    const v2 = map.at(r2, c2);
    return v == v2;
}

fn solvePart2(data: []const u8) !void {
    const map = try Map.new(data);

    var total: usize = 0;

    var r: isize = 1;
    while (r < map.height - 1) : (r += 1) {
        var c: isize = 1;
        while (c < map.width - 1) : (c += 1) {
            if (map.at(r, c) != 'A') {
                continue;
            }

            if (!isMOrS(&map, r - 1, c - 1) or !isMOrS(&map, r + 1, c + 1) or areEqual(&map, r - 1, c - 1, r + 1, c + 1)) {
                continue;
            }

            if (!isMOrS(&map, r + 1, c - 1) or !isMOrS(&map, r - 1, c + 1) or areEqual(&map, r + 1, c - 1, r - 1, c + 1)) {
                continue;
            }
            total += 1;
        }
    }

    std.debug.print("Part 2 total = {}\n", .{total});
}

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var allocator = gpa.allocator();
    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak) {
            std.log.err("You have a memory leak. You're useless.\n", .{});
        }
    }

    const file = std.fs.cwd().openFile("./input.txt", .{}) catch |err| {
        std.log.err("Could not find file: {}", .{err});
        return;
    };
    defer file.close();

    const input = file.reader().readAllAlloc(allocator, std.math.maxInt(usize)) catch |err| {
        std.log.err("Could not read file: {}", .{err});
        return;
    };
    defer allocator.free(input);

    solvePart1(input) catch |err| {
        std.log.err("Error solving part 1: {}", .{err});
        return;
    };

    solvePart2(input) catch |err| {
        std.log.err("Error solving part 2: {}", .{err});
        return;
    };
}
