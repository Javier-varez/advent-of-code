const std = @import("std");

const Map = struct {
    data: []const u8,
    width: usize,
    height: usize,
    stride: usize,

    pub fn init(data: []const u8) Map {
        const width = for (data, 0..) |byte, idx| {
            if (byte == '\n') break idx;
        } else unreachable();

        const stride = width + 1;
        const height = data.len / stride;

        return Map{ .data = data, .width = width, .height = height, .stride = stride };
    }

    pub fn at(self: *const Map, r: usize, c: usize) u8 {
        return self.data[r * self.stride + c] - '0';
    }
};

const Loc = struct {
    r: usize,
    c: usize,
};

pub fn countTrails(r: usize, c: usize, next_node: u8, map: Map, solutions: *std.AutoHashMap(Loc, void)) !void {
    if (next_node >= 10) {
        try solutions.put(Loc{ .r = r, .c = c }, {});
    }

    if (r > 0 and next_node == map.at(r - 1, c)) {
        try countTrails(r - 1, c, next_node + 1, map, solutions);
    }
    if (r < (map.height - 1) and next_node == map.at(r + 1, c)) {
        try countTrails(r + 1, c, next_node + 1, map, solutions);
    }
    if (c > 0 and next_node == map.at(r, c - 1)) {
        try countTrails(r, c - 1, next_node + 1, map, solutions);
    }
    if (c < (map.width - 1) and next_node == map.at(r, c + 1)) {
        try countTrails(r, c + 1, next_node + 1, map, solutions);
    }
}

pub fn part1(map: Map, allocator: std.mem.Allocator) !void {
    var result: usize = 0;
    for (0..map.height) |r| {
        for (0..map.width) |c| {
            if (map.at(r, c) == 0) {
                var solutions = std.AutoHashMap(Loc, void).init(allocator);
                defer solutions.deinit();

                try countTrails(r, c, 1, map, &solutions);
                std.log.debug("Found trailhead at {}, {}: score {}", .{ r, c, solutions.count() });
                result += solutions.count();
            }
        }
    }
    std.log.debug("Part 1 solution is {}", .{result});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const result = gpa.deinit();
        if (result == .leak) {
            std.log.err("Memory leak!", .{});
        }
    }
    const allocator = gpa.allocator();

    const data = try std.fs.cwd().readFileAlloc(allocator, "./realinput.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    try part1(Map.init(data), allocator);
}
