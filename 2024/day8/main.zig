const std = @import("std");

const DimError = error{DimError};

const Freq = u8;

const Map = struct {
    width: isize,
    height: isize,

    antennasByFrequency: AntennasByFrequency,
    antennasByLocation: AntennasByLocation,

    const AntennasByLocation = std.AutoHashMap(Location, Freq);
    const AntennasByFrequency = std.AutoHashMap(u8, std.ArrayList(Location));

    fn init(allocator: std.mem.Allocator, data: []u8) !Map {
        const total_length = data.len;

        const width = for (data, 0..) |byte, idx| {
            if (byte == '\n') {
                break idx;
            }
        } else {
            return DimError.DimError;
        };

        const stride = width + 1;
        const height = total_length / stride;

        var antennasByLocation = AntennasByLocation.init(allocator);
        errdefer antennasByLocation.deinit();
        var antennasByFrequency = AntennasByFrequency.init(allocator);
        errdefer {
            var iter = antennasByFrequency.valueIterator();
            while (iter.next()) |v| {
                v.deinit();
            }
            antennasByFrequency.deinit();
        }

        for (0..width) |c| {
            for (0..height) |r| {
                const freq = data[c + r * stride];
                if (freq != '.') {
                    const loc = Location{ .r = @intCast(r), .c = @intCast(c) };
                    try antennasByLocation.put(loc, freq);

                    const putResult = try antennasByFrequency.getOrPut(freq);
                    if (!putResult.found_existing) {
                        putResult.value_ptr.* = std.ArrayList(Location).init(allocator);
                    }
                    try putResult.value_ptr.append(loc);
                }
            }
        }

        return Map{ .width = @intCast(width), .height = @intCast(height), .antennasByLocation = antennasByLocation, .antennasByFrequency = antennasByFrequency };
    }

    fn deinit(self: *Map) void {
        self.antennasByLocation.deinit();
        var iter = self.antennasByFrequency.valueIterator();
        while (iter.next()) |v| {
            v.deinit();
        }
        self.antennasByFrequency.deinit();
    }

    fn hasNoFreqConflict(self: *const Map, location: Location, freq: u8) bool {
        if (self.antennasByLocation.get(location)) |antennaFreq| {
            return antennaFreq != freq;
        }
        return true;
    }
};

const Location = struct {
    r: isize,
    c: isize,

    fn isValid(self: *const Location, map: *const Map) bool {
        return self.r >= 0 and self.c >= 0 and self.r < map.height and self.c < map.width;
    }
};

fn logMap(map: *const Map) void {
    std.log.debug("Antennas by location:", .{});
    {
        var iter = map.antennasByLocation.iterator();
        while (iter.next()) |kv| {
            std.log.debug("\t[{}] = {}", .{ kv.key_ptr.*, kv.value_ptr.* });
        }
    }

    std.log.debug("Antennas by frequency:", .{});
    {
        var iter = map.antennasByFrequency.iterator();
        while (iter.next()) |kv| {
            const freq = kv.key_ptr.*;
            std.log.debug("\tfreq: {}", .{freq});
            for (kv.value_ptr.items) |loc| {
                std.log.debug("\t\t{}", .{loc});
            }
        }
    }
}

fn part1(allocator: std.mem.Allocator, map: *const Map) !void {
    var antinodes = std.AutoHashMap(Location, void).init(allocator);
    defer antinodes.deinit();

    var antennaSrcIter = map.antennasByLocation.iterator();
    while (antennaSrcIter.next()) |antennaSrc| {
        var antennaDestIter = map.antennasByLocation.iterator();
        while (antennaDestIter.next()) |antennaDest| {
            if (antennaSrc.key_ptr.r == antennaDest.key_ptr.r and antennaSrc.key_ptr.c == antennaDest.key_ptr.c) {
                // Ignore antennas at the same location
                continue;
            }

            const srcFreq = antennaSrc.value_ptr.*;
            const destFreq = antennaDest.value_ptr.*;
            if (srcFreq != destFreq) {
                // The frequencies of the antennas do not match
                continue;
            }

            const srcLoc = antennaSrc.key_ptr.*;
            const destLoc = antennaDest.key_ptr.*;

            const distanceR = destLoc.r - srcLoc.r;
            const distanceC = destLoc.c - srcLoc.c;

            const firstAntinode = Location{ .r = destLoc.r + distanceR, .c = destLoc.c + distanceC };
            if (firstAntinode.isValid(map) and map.hasNoFreqConflict(firstAntinode, srcFreq)) {
                try antinodes.put(firstAntinode, {});
            }
            const secondAntinode = Location{ .r = srcLoc.r - distanceR, .c = srcLoc.c - distanceC };
            if (secondAntinode.isValid(map) and map.hasNoFreqConflict(secondAntinode, srcFreq)) {
                try antinodes.put(secondAntinode, {});
            }
        }
    }
    std.debug.print("Unique antinode locations: {}\n", .{antinodes.count()});
}

fn part2(allocator: std.mem.Allocator, map: *const Map) !void {
    var antinodes = std.AutoHashMap(Location, void).init(allocator);
    defer antinodes.deinit();

    var antennaSrcIter = map.antennasByLocation.iterator();
    while (antennaSrcIter.next()) |antennaSrc| {
        var antennaDestIter = map.antennasByLocation.iterator();
        while (antennaDestIter.next()) |antennaDest| {
            if (antennaSrc.key_ptr.r == antennaDest.key_ptr.r and antennaSrc.key_ptr.c == antennaDest.key_ptr.c) {
                // Ignore antennas at the same location
                continue;
            }

            const srcFreq = antennaSrc.value_ptr.*;
            const destFreq = antennaDest.value_ptr.*;
            if (srcFreq != destFreq) {
                // The frequencies of the antennas do not match
                continue;
            }

            const srcLoc = antennaSrc.key_ptr.*;
            const destLoc = antennaDest.key_ptr.*;

            const distanceR = destLoc.r - srcLoc.r;
            const distanceC = destLoc.c - srcLoc.c;

            // antinodes after dest loc
            var antinodeLoc = Location{ .r = destLoc.r, .c = destLoc.c };
            while (antinodeLoc.isValid(map)) {
                try antinodes.put(antinodeLoc, {});
                antinodeLoc = Location{ .r = antinodeLoc.r + distanceR, .c = antinodeLoc.c + distanceC };
            }

            // antinodes before src loc
            antinodeLoc = Location{ .r = srcLoc.r, .c = srcLoc.c };
            while (antinodeLoc.isValid(map)) {
                try antinodes.put(antinodeLoc, {});
                antinodeLoc = Location{ .r = antinodeLoc.r - distanceR, .c = antinodeLoc.c - distanceC };
            }
        }
    }
    std.debug.print("Unique antinode locations: {}\n", .{antinodes.count()});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        const result = gpa.deinit();
        if (result == .leak) {
            std.log.err("Memory leak\n", .{});
        }
    }

    const input = try std.fs.cwd().readFileAlloc(allocator, "realinput.txt", std.math.maxInt(usize));
    defer allocator.free(input);
    var map = try Map.init(allocator, input);
    defer map.deinit();
    logMap(&map);
    try part1(allocator, &map);
    try part2(allocator, &map);
}
