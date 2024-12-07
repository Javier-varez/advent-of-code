const std = @import("std");

const ParseError = error{
    DelimiterNotFound,
};

fn findDelimiter(str: []const u8, delimiter: u8) ?usize {
    for (str, 0..) |c, idx| {
        if (c == delimiter) return idx;
    }
    return null;
}

const Input = struct {
    constraints: std.AutoHashMap(usize, std.ArrayList(usize)),
    updateSets: std.ArrayList(std.ArrayList(usize)),

    fn fromFile(allocator: std.mem.Allocator, path: []const u8) !Input {
        var parsingUpdates = false;

        var constraints = std.AutoHashMap(usize, std.ArrayList(usize)).init(allocator);
        errdefer {
            var iter = constraints.iterator();
            while (iter.next()) |kv| {
                kv.value_ptr.deinit();
            }
            constraints.deinit();
        }

        var updateSets = std.ArrayList(std.ArrayList(usize)).init(allocator);
        errdefer {
            for (updateSets.items) |v| {
                v.deinit();
            }
            updateSets.deinit();
        }

        var file = try std.fs.cwd().openFile(path, .{});
        while (try file.reader().readUntilDelimiterOrEofAlloc(allocator, '\n', std.math.maxInt(usize))) |line| {
            defer allocator.free(line);

            if (std.mem.eql(u8, line, "")) {
                parsingUpdates = true;
                continue;
            }

            if (!parsingUpdates) {
                const delimiterIdx =
                    for (line, 0..) |c, idx|
                {
                    if (c == '|') break idx;
                } else {
                    return ParseError.DelimiterNotFound;
                };

                const firstPage = try std.fmt.parseUnsigned(usize, line[0..delimiterIdx], 10);
                const secondPage = try std.fmt.parseUnsigned(usize, line[delimiterIdx + 1 ..], 10);
                var innerConstraint = try constraints.getOrPut(firstPage);
                if (!innerConstraint.found_existing) {
                    innerConstraint.value_ptr.* = std.ArrayList(usize).init(allocator);
                }
                try innerConstraint.value_ptr.append(secondPage);
            } else {
                var updateSet = std.ArrayList(usize).init(allocator);

                var unparsed = line[0..];
                while (findDelimiter(unparsed, ',')) |delimiter| {
                    const value = try std.fmt.parseUnsigned(usize, unparsed[0..delimiter], 10);
                    try updateSet.append(value);
                    unparsed = unparsed[delimiter + 1 ..];
                }

                const value = try std.fmt.parseUnsigned(usize, unparsed, 10);
                try updateSet.append(value);
                try updateSets.append(updateSet);
            }
        }

        return Input{ .constraints = constraints, .updateSets = updateSets };
    }

    fn deinit(self: *Input) void {
        var iter = self.constraints.iterator();
        while (iter.next()) |kv| {
            kv.value_ptr.deinit();
        }
        self.constraints.deinit();

        for (self.updateSets.items) |updateSet| {
            updateSet.deinit();
        }
        self.updateSets.deinit();
    }
};

fn findIncorrectOrder(allocator: std.mem.Allocator, updateSet: []const usize, input: *const Input) !?struct { first: usize, second: usize } {
    var updatedPages = std.AutoHashMap(usize, usize).init(allocator);
    defer updatedPages.deinit();

    for (updateSet, 0..) |page, cur_idx| {
        if (input.constraints.get(page)) |constraints| {
            const maybeIncorrectlyOrderedPage = for (constraints.items) |pageAfter| {
                if (updatedPages.get(pageAfter)) |idx| {
                    break idx;
                }
            } else null;

            if (maybeIncorrectlyOrderedPage) |incorrectlyOrderedPage| {
                return .{ .first = incorrectlyOrderedPage, .second = cur_idx };
            }
        }

        try updatedPages.put(page, cur_idx);
    }

    return null;
}

pub fn part1(allocator: std.mem.Allocator, input: *const Input) !void {
    var count: usize = 0;
    outer: for (input.updateSets.items) |updateSet| {
        var updatedPages = std.AutoHashMap(usize, void).init(allocator);
        defer updatedPages.deinit();

        for (updateSet.items) |page| {
            if (input.constraints.get(page)) |constraints| {
                const isValid = for (constraints.items) |pageAfter| {
                    if (updatedPages.contains(pageAfter)) {
                        break false;
                    }
                } else true;

                if (!isValid) {
                    continue :outer;
                }
            }

            try updatedPages.put(page, {});
        }

        const idx = updateSet.items.len / 2;
        count += updateSet.items[idx];
    }

    std.debug.print("Part 1 result is {}\n", .{count});
}

pub fn part2(allocator: std.mem.Allocator, input: *const Input) !void {
    var count: usize = 0;
    for (input.updateSets.items) |updateSet| {
        var mutableUpdateSet = try updateSet.clone();
        defer mutableUpdateSet.deinit();

        var wasIncorrectlyOrdered = false;
        while (try findIncorrectOrder(allocator, mutableUpdateSet.items, input)) |v| {
            const temp = mutableUpdateSet.items[v.first];
            mutableUpdateSet.items[v.first] = mutableUpdateSet.items[v.second];
            mutableUpdateSet.items[v.second] = temp;

            wasIncorrectlyOrdered = true;
        }

        if (wasIncorrectlyOrdered) {
            const idx = mutableUpdateSet.items.len / 2;
            count += mutableUpdateSet.items[idx];
        }
    }

    std.debug.print("Part 2 result is {}\n", .{count});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        const deinit_result = gpa.deinit();
        if (deinit_result == .leak) {
            std.log.err("There was a memory leak!", .{});
        }
    }
    var input = try Input.fromFile(allocator, "./realinput.txt");
    defer input.deinit();

    try part1(allocator, &input);
    try part2(allocator, &input);
}
