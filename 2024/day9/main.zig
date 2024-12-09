const std = @import("std");

fn part1(allocator: std.mem.Allocator) !void {
    const data = try std.fs.cwd().readFileAlloc(allocator, "./realinput.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    var i: usize = 0;
    // -2 to account for the last \n
    var j: usize = data.len - 2;

    var expanded_blocks = std.ArrayList(usize).init(allocator);
    defer expanded_blocks.deinit();
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

fn attemptMove(blocks: []?usize, id_base: usize, num_blocks: usize) void {
    var maybe_empty_block_base: ?usize = null;
    var empty_block_size: usize = 0;
    const empty_block_base = for (0..id_base) |i| {
        if (blocks[i] == null) {
            if (maybe_empty_block_base == null) {
                maybe_empty_block_base = i;
                empty_block_size = 1;
            } else {
                empty_block_size += 1;
            }
        } else {
            maybe_empty_block_base = null;
            empty_block_size = 0;
        }

        if (empty_block_size >= num_blocks) {
            break maybe_empty_block_base.?;
        }
    } else null;

    if (empty_block_base) |empty_base| {
        const id = blocks[id_base].?;
        for (id_base..id_base + num_blocks) |idx| {
            blocks[idx] = null;
        }
        for (empty_base..empty_base + num_blocks) |idx| {
            blocks[idx] = id;
        }
    }
}

fn part2(allocator: std.mem.Allocator) !void {
    const data = try std.fs.cwd().readFileAlloc(allocator, "./realinput.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    const trimmed_data = std.mem.trim(u8, data, &[4]u8{ '\n', ' ', '\t', '\r' });

    var expanded_blocks = std.ArrayList(?usize).init(allocator);
    defer expanded_blocks.deinit();

    // Do a first pass to expand it all
    var max_file_id: usize = 0;
    for (trimmed_data, 0..) |byte, idx| {
        const num_blocks: usize = byte - '0';
        var file_id: ?usize = null;

        if (idx % 2 == 0) {
            file_id = idx >> 1;
            max_file_id = idx >> 1;
        }

        for (0..num_blocks) |_| {
            try expanded_blocks.append(file_id);
        }
    }

    // Do a second pass to reorder
    var i = expanded_blocks.items.len - 1;
    var cur_file_id = max_file_id;
    while (cur_file_id > 0) {
        // Look for the first value with this ID.
        while (expanded_blocks.items[i] != cur_file_id) {
            i -= 1;
        }
        var num_blocks: usize = 0;
        while (expanded_blocks.items[i] == cur_file_id) {
            num_blocks += 1;
            i -= 1;
        }

        const id_base = i + 1;
        // std.log.debug("Attempting to move id {} at {}, blocks {}", .{ cur_file_id, id_base, num_blocks });

        attemptMove(expanded_blocks.items, id_base, num_blocks);

        // std.log.debug("Result {any}", .{expanded_blocks.items});

        cur_file_id -= 1;
    }

    var csum: usize = 0;
    for (expanded_blocks.items, 0..) |maybe_file_id, pos| {
        if (maybe_file_id) |file_id| {
            csum += file_id * pos;
        }
    }

    std.log.debug("part 2: {any}", .{csum});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        const result = gpa.deinit();
        if (result == .leak) {
            std.log.err("memory was leaked", .{});
        }
    }

    try part1(allocator);
    try part2(allocator);
}
