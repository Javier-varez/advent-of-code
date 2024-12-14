const std = @import("std");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        const result = gpa.deinit();
        if (result == .leak) {
            std.log.err("Memory leaks!", .{});
        }
    }
    const allocator = gpa.allocator();

    const data = try std.fs.cwd().readFileAlloc(allocator, "realinput.txt", std.math.maxInt(usize));
    defer allocator.free(data);
    try part1(allocator, data);
    try part2(allocator, data);
}

const Location = struct {
    x: isize,
    y: isize,
};

const Velocity = struct {
    dx: isize,
    dy: isize,
};

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

const Robot = struct {
    location: Location,
    velocity: Velocity,

    fn move(self: *Robot, seconds: usize) void {
        const times: isize = @intCast(seconds);

        const x = @mod(self.location.x + self.velocity.dx * times, WIDTH);
        // if (x < 0) {
        //     x = WIDTH + x;
        // }

        const y = @mod(self.location.y + self.velocity.dy * times, HEIGHT);
        // if (y < 0) {
        //     y = HEIGHT + y;
        // }
        self.location.x = x;
        self.location.y = y;
    }
};

pub fn parseRobots(allocator: std.mem.Allocator, data: []const u8) !std.ArrayList(Robot) {
    var robots = std.ArrayList(Robot).init(allocator);

    var lineIter = std.mem.splitScalar(u8, data, '\n');
    while (lineIter.next()) |line| {
        if (line.len == 0) {
            continue;
        }
        var iter = std.mem.splitScalar(u8, line, ' ');
        const loc = iter.next().?;
        const vel = iter.next().?;

        var locIter = std.mem.splitScalar(u8, loc[2..], ',');
        const locX = try std.fmt.parseInt(isize, locIter.next().?, 0);
        const locY = try std.fmt.parseInt(isize, locIter.next().?, 0);

        var velIter = std.mem.splitScalar(u8, vel[2..], ',');
        const velX = try std.fmt.parseInt(isize, velIter.next().?, 0);
        const velY = try std.fmt.parseInt(isize, velIter.next().?, 0);

        try robots.append(Robot{
            .location = Location{
                .x = locX,
                .y = locY,
            },
            .velocity = Velocity{
                .dx = velX,
                .dy = velY,
            },
        });
    }
    return robots;
}

pub fn part1(allocator: std.mem.Allocator, data: []const u8) !void {
    const SECONDS: usize = 100;

    var robots = try parseRobots(allocator, data);
    defer robots.deinit();

    var quads = [4]usize{ 0, 0, 0, 0 };

    for (robots.items) |*robot| {
        // std.log.debug("Initial position {},{}", .{ robot.location.x, robot.location.y });
        robot.move(SECONDS);
        // std.log.debug("Final position {},{}", .{ robot.location.x, robot.location.y });

        if (robot.location.x == WIDTH / 2) {
            continue;
        }
        const qx: usize = @intFromBool(robot.location.x > (WIDTH / 2));

        if (robot.location.y == HEIGHT / 2) {
            continue;
        }
        const qy: usize = @intFromBool(robot.location.y > (HEIGHT / 2));
        const q = 2 * qy + qx;

        quads[q] += 1;
        // std.log.debug("Quad {}", .{q});
    }

    std.log.debug("Result {},{},{},{}", .{ quads[0], quads[1], quads[2], quads[3] });
    std.log.debug("Result {}", .{quads[0] * quads[1] * quads[2] * quads[3]});
}

pub fn part2(allocator: std.mem.Allocator, data: []const u8) !void {
    var robots = try parseRobots(allocator, data);
    defer robots.deinit();

    var elapsed: usize = 0;
    while (true) {
        var quads = [4]usize{ 0, 0, 0, 0 };
        var robotMap = std.AutoHashMap(Location, void).init(allocator);
        defer robotMap.deinit();

        for (robots.items) |*robot| {
            robot.move(1);
            try robotMap.put(robot.location, {});
            if (robot.location.x == WIDTH / 2) {
                continue;
            }
            const qx: usize = @intFromBool(robot.location.x > (WIDTH / 2));

            if (robot.location.y == HEIGHT / 2) {
                continue;
            }
            const qy: usize = @intFromBool(robot.location.y > (HEIGHT / 2));
            const q = 2 * qy + qx;

            quads[q] += 1;
        }

        for (0..HEIGHT) |y| {
            for (0..WIDTH) |x| {
                var c: u8 = ' ';
                const l = Location{ .x = @intCast(x), .y = @intCast(y) };
                if (robotMap.get(l) != null) {
                    c = 'x';
                }
                std.debug.print("{c}", .{c});
            }
            std.debug.print("\n", .{});
        }

        elapsed += 1;
        for (quads) |q| {
            // I made the gross assumption that the tree will likely mostly form in one of the 4
            // quadrants. Use that to try to figure out when the entropy drops, then print the time and the tree
            if (q >= (robots.items.len * 7 / 10)) {
                for (0..HEIGHT) |y| {
                    for (0..WIDTH) |x| {
                        var c: u8 = ' ';
                        const l = Location{ .x = @intCast(x), .y = @intCast(y) };
                        if (robotMap.get(l) != null) {
                            c = 'x';
                        }
                        std.debug.print("{c}", .{c});
                    }
                    std.debug.print("\n", .{});
                }
                std.debug.print("Entropy dropped at time {} seconds\n", .{elapsed});
                return;
            }
        }
    }
}
