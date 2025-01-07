const std = @import("std");

fn emulate_encode_str(data: []const u8) usize {
    var i: usize = 0;
    var encoded: usize = 0;
    while (i < data.len) {
        const char = data[i];
        i += 1;
        switch (char) {
            '"' => {
                encoded += 2;
            },
            '\\' => {
                encoded += 2;
            },
            else => {
                encoded += 1;
            },
        }
    }
    return encoded - data.len;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const stat = gpa.deinit();
        if (stat == .leak) {
            std.log.err("memiry leak", .{});
        }
    }

    const allocator = gpa.allocator();
    const data = try std.fs.cwd().readFileAlloc(allocator, "input.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    var iter = std.mem.splitScalar(u8, data, '\n');
    var result: usize = 0;
    while (iter.next()) |line| {
        if (line.len == 0) continue;
        const diff = emulate_encode_str(line);
        result += diff + 2;
    }

    std.debug.print("{}", .{result});
}
