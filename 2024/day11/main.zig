const std = @import("std");

fn countDigits(num: usize) usize {
    var digits: usize = 0;
    var n = num;
    while (n != 0) {
        n = n / 10;
        digits += 1;
    }
    return digits;
}

fn splitNumber(num: usize, digits: usize) struct { left: usize, right: usize } {
    var right: usize = 0;
    var left: usize = num;
    for (0..digits / 2) |i| {
        var right_digit = left % 10;
        for (0..i) |_| {
            right_digit *= 10;
        }
        right += right_digit;
        left /= 10;
    }

    return .{ .right = right, .left = left };
}

const K = struct {
    num: usize,
    blinks: usize,
};

fn resolveBlinks(memory: *std.AutoHashMap(K, usize), num: usize, blinks: usize) !usize {
    if (memory.get(K{ .num = num, .blinks = blinks })) |solution| {
        return solution;
    }

    if (blinks == 0) {
        return 1;
    }

    const num_digits = countDigits(num);
    var solution: usize = 0;
    if (num == 0) {
        solution = try resolveBlinks(memory, 1, blinks - 1);
    } else if (num_digits % 2 == 0) {
        const split = splitNumber(num, num_digits);
        const left = try resolveBlinks(memory, split.left, blinks - 1);
        const right = try resolveBlinks(memory, split.right, blinks - 1);
        solution = left + right;
    } else {
        solution = try resolveBlinks(memory, num * 2024, blinks - 1);
    }

    try memory.put(K{ .num = num, .blinks = blinks }, solution);
    return solution;
}

fn resolve(allocator: std.mem.Allocator, data: []u8, blinks: usize) !void {
    var memory = std.AutoHashMap(K, usize).init(allocator);
    defer memory.deinit();

    var total: usize = 0;
    {
        var iter = std.mem.split(u8, data, &[1]u8{' '});
        while (iter.next()) |value| {
            const num_string = std.mem.trim(u8, value, &[1]u8{'\n'});
            const num = try std.fmt.parseUnsigned(usize, num_string, 0);
            total += try resolveBlinks(&memory, num, blinks);
        }
    }
    std.log.debug("Solution is {}", .{total});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }

    const data = try std.fs.cwd().readFileAlloc(allocator, "realinput.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    try resolve(allocator, data, 25);
    try resolve(allocator, data, 75);
}
