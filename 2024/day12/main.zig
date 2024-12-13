const std = @import("std");

const Map = struct {
    width: usize,
    height: usize,
    stride: usize,
    data: []const u8,

    fn init(data: []const u8) Map {
        const width = for (data, 0..) |byte, idx| {
            if (byte == '\n') {
                break idx;
            }
            // Yeah, things can definitely never fail
        } else unreachable;

        const stride = width + 1;
        const height = data.len / stride;

        return Map{ .height = height, .width = width, .stride = stride, .data = data };
    }

    fn at(self: *const Map, l: Location) u8 {
        if (l.r < 0 or l.r >= self.height or l.c < 0 or l.c >= self.width) {
            return 0; // Not a char
        }
        const r: usize = @intCast(l.r);
        const c: usize = @intCast(l.c);
        return self.data[r * self.stride + c];
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const err = gpa.deinit();
        if (err == .leak) {
            std.log.err("Memory leak!", .{});
        }
    }
    const allocator = gpa.allocator();
    const data = try std.fs.cwd().readFileAlloc(allocator, "realinput.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    const map = Map.init(data);
    try part1(allocator, map);
    try part2(allocator, map);
}

const Location = struct {
    r: isize,
    c: isize,

    fn up(self: Location) Location {
        return Location{ .r = self.r - 1, .c = self.c };
    }
    fn down(self: Location) Location {
        return Location{ .r = self.r + 1, .c = self.c };
    }
    fn left(self: Location) Location {
        return Location{ .r = self.r, .c = self.c - 1 };
    }
    fn right(self: Location) Location {
        return Location{ .r = self.r, .c = self.c + 1 };
    }
};

fn calcAreaAndPerimeter(handled: *std.AutoHashMap(Location, void), map: Map, curLoc: Location, byte: u8) !struct { area: usize, perimeter: usize } {
    try handled.put(curLoc, {});

    var area: usize = 1;
    var perimeter: usize = 0;

    if (curLoc.r != 0 and byte == map.at(curLoc.up())) {
        if (handled.get(curLoc.up()) == null) {
            const part = try calcAreaAndPerimeter(handled, map, curLoc.up(), byte);
            area += part.area;
            perimeter += part.perimeter;
        }
    } else {
        perimeter += 1;
    }

    if (curLoc.r != (map.height - 1) and byte == map.at(curLoc.down())) {
        if (handled.get(curLoc.down()) == null) {
            const part = try calcAreaAndPerimeter(handled, map, curLoc.down(), byte);
            area += part.area;
            perimeter += part.perimeter;
        }
    } else {
        perimeter += 1;
    }

    if (curLoc.c != 0 and byte == map.at(curLoc.left())) {
        if (handled.get(curLoc.left()) == null) {
            const part = try calcAreaAndPerimeter(handled, map, curLoc.left(), byte);
            area += part.area;
            perimeter += part.perimeter;
        }
    } else {
        perimeter += 1;
    }

    if (curLoc.c != (map.width - 1) and byte == map.at(curLoc.right())) {
        if (handled.get(curLoc.right()) == null) {
            const part = try calcAreaAndPerimeter(handled, map, curLoc.right(), byte);
            area += part.area;
            perimeter += part.perimeter;
        }
    } else {
        perimeter += 1;
    }

    return .{ .perimeter = perimeter, .area = area };
}

fn part1(allocator: std.mem.Allocator, map: Map) !void {
    var handled = std.AutoHashMap(Location, void).init(allocator);
    defer handled.deinit();

    var total: usize = 0;
    for (0..map.height) |r| {
        for (0..map.width) |c| {
            const curLoc = Location{ .r = @intCast(r), .c = @intCast(c) };
            if (handled.get(curLoc) != null) {
                continue;
            }

            const byte = map.at(curLoc);
            const part = try calcAreaAndPerimeter(&handled, map, curLoc, byte);
            std.log.debug("area for byte {}: {}, perimeter: {}", .{ byte, part.area, part.perimeter });
            total += part.area * part.perimeter;
        }
    }
    std.log.debug("Result is {}", .{total});
}

fn calcAreaAndSides(handled: *std.AutoHashMap(Location, void), map: Map, curLoc: Location, byte: u8) !struct { area: usize, sides: usize } {
    try handled.put(curLoc, {});

    var area: usize = 1;
    var sides: usize = 0;

    if (byte == map.at(curLoc.up()) and byte == map.at(curLoc.right()) and byte != map.at(curLoc.up().right())) {
        sides += 1;
    }

    if (byte != map.at(curLoc.up()) and byte != map.at(curLoc.right())) {
        sides += 1;
    }

    if (byte == map.at(curLoc.down()) and byte == map.at(curLoc.right()) and byte != map.at(curLoc.down().right())) {
        sides += 1;
    }

    if (byte != map.at(curLoc.down()) and byte != map.at(curLoc.right())) {
        sides += 1;
    }

    if (byte == map.at(curLoc.down()) and byte == map.at(curLoc.left()) and byte != map.at(curLoc.down().left())) {
        sides += 1;
    }

    if (byte != map.at(curLoc.down()) and byte != map.at(curLoc.left())) {
        sides += 1;
    }

    if (byte == map.at(curLoc.up()) and byte == map.at(curLoc.left()) and byte != map.at(curLoc.up().left())) {
        sides += 1;
    }

    if (byte != map.at(curLoc.up()) and byte != map.at(curLoc.left())) {
        sides += 1;
    }

    if (byte == map.at(curLoc.up()) and handled.get(curLoc.up()) == null) {
        const part = try calcAreaAndSides(handled, map, curLoc.up(), byte);
        area += part.area;
        sides += part.sides;
    }

    if (byte == map.at(curLoc.down()) and handled.get(curLoc.down()) == null) {
        const part = try calcAreaAndSides(handled, map, curLoc.down(), byte);
        area += part.area;
        sides += part.sides;
    }

    if (byte == map.at(curLoc.left()) and handled.get(curLoc.left()) == null) {
        const part = try calcAreaAndSides(handled, map, curLoc.left(), byte);
        area += part.area;
        sides += part.sides;
    }

    if (byte == map.at(curLoc.right()) and handled.get(curLoc.right()) == null) {
        const part = try calcAreaAndSides(handled, map, curLoc.right(), byte);
        area += part.area;
        sides += part.sides;
    }

    return .{ .sides = sides, .area = area };
}

fn part2(allocator: std.mem.Allocator, map: Map) !void {
    var handled = std.AutoHashMap(Location, void).init(allocator);
    defer handled.deinit();

    var total: usize = 0;
    for (0..map.height) |r| {
        for (0..map.width) |c| {
            const curLoc = Location{ .r = @intCast(r), .c = @intCast(c) };
            if (handled.get(curLoc) != null) {
                continue;
            }

            const byte = map.at(curLoc);
            const part = try calcAreaAndSides(&handled, map, curLoc, byte);
            std.log.debug("area for byte {}: {}, sides: {}", .{ byte, part.area, part.sides });
            total += part.area * part.sides;
        }
    }
    std.log.debug("Result is {}", .{total});
}
