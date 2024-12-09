const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const data = try std.fs.cwd().readFileAlloc(allocator, "./realinput.txt", std.math.maxInt(usize));

    var i: usize = 0;
    // -2 to account for the last \n
    var j: usize = data.len - 2;

    var expanded_blocks = std.ArrayList(usize).init(allocator);
    while (i <= j) {
        if (i % 2 == 0) {
            // Actual file
            const index = i >> 1;
            const num_blocks: usize = data[i] - '0';

            for (0..num_blocks) |_| {
                try expanded_blocks.append(index);
            }
            i += 1;
        } else {
            // Free space, we take some blocks from the end
            const index = j >> 1;
            const num_blocks: usize = data[j] - '0';
            const num_free_spaces: usize = data[i] - '0';

            if (num_blocks == 0) {
                j -= 2;
            } else if (num_free_spaces == 0) {
                i += 1;
            } else {
                try expanded_blocks.append(index);
                data[j] = @intCast((num_blocks - 1) + '0');
                data[i] = @intCast((num_free_spaces - 1) + '0');
            }
        }
    }

    var csum: usize = 0;
    for (expanded_blocks.items, 0..) |file_id, pos| {
        csum += file_id * pos;
    }

    std.log.debug("part 1: {}", .{csum});
}
