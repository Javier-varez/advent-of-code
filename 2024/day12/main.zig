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
        return self.data[l.r * self.stride + l.c];
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
}

const Location = struct {
    r: usize,
    c: usize,

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
            const curLoc = Location{ .r = r, .c = c };
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
