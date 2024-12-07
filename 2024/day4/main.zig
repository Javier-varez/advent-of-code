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

    std.debug.print("total = {}\n", .{total});
}

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    var allocator = gpa.allocator();
    defer {
        const deinit_status = gpa.deinit();
        if (deinit_status == .leak) {}
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
}
