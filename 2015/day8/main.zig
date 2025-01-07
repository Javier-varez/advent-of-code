const std = @import("std");

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

    var i: usize = 0;
    var c: usize = 0;
    var u: usize = 0;
    while (i < data.len) {
        var char = data[i];
        i += 1;
        switch (char) {
            '"' => {
                u += 1;
            },
            '\n' => {},
            '\\' => {
                u += 1;
                char = data[i];
                if (char == 'x') {
                    u += 3;
                    i += 3;
                } else {
                    u += 1;
                    i += 1;
                }
                c += 1;
            },
            else => {
                u += 1;
                c += 1;
            },
        }
    }

    std.debug.print("{}", .{u - c});
}
