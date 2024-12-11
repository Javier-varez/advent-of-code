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

const List = std.DoublyLinkedList(usize);
const Node = std.DoublyLinkedList(usize).Node;

fn runStep(allocator: std.mem.Allocator, list: *List) !void {
    var maybe_node = list.first;
    while (maybe_node) |node| {
        if (node.data == 0) {
            node.data = 1;
            maybe_node = node.next;
            continue;
        }

        const num_digits = countDigits(node.data);
        if (num_digits % 2 == 0) {
            const split = splitNumber(node.data, num_digits);
            node.data = split.right;

            const left_node = try allocator.create(Node);
            left_node.data = split.left;
            list.insertBefore(node, left_node);
        } else {
            node.data *= 2024;
        }

        maybe_node = node.next;
    }
}

fn part1(allocator: std.mem.Allocator, data: []u8) !void {
    var list = std.DoublyLinkedList(usize){};
    defer {
        var maybe_node = list.first;
        while (maybe_node) |node| {
            maybe_node = node.next;
            allocator.destroy(node);
        }
    }

    {
        var iter = std.mem.split(u8, data, &[1]u8{' '});
        while (iter.next()) |value| {
            const num_string = std.mem.trim(u8, value, &[1]u8{'\n'});
            const num = try std.fmt.parseUnsigned(usize, num_string, 0);
            var node = try allocator.alloc(Node, 1);
            node[0].data = num;
            list.append(&node[0]);
        }
    }

    for (0..25) |_| {
        try runStep(allocator, &list);
    }

    var count: usize = 0;
    {
        var maybe_node = list.first;
        while (maybe_node) |node| {
            count += 1;
            maybe_node = node.next;
        }
    }

    std.log.debug("part 1, {}", .{count});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        _ = gpa.deinit();
    }
    const allocator = gpa.allocator();

    const data = try std.fs.cwd().readFileAlloc(allocator, "realinput.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    try part1(allocator, data);
}
