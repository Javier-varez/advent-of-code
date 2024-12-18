const std = @import("std");

const MachineState = struct {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    instrs: std.ArrayList(u3),
    out: std.ArrayList(usize),

    fn deinit(self: *const MachineState) void {
        self.instrs.deinit();
        self.out.deinit();
    }
};

pub fn parseReg(line: []const u8) usize {
    var iter = std.mem.splitScalar(u8, line, ':');
    _ = iter.next();
    return std.fmt.parseInt(usize, std.mem.trim(u8, iter.next().?, &[4]u8{ ' ', '\n', '\t', '\r' }), 0) catch {
        @panic("Unable to parse register integer number");
    };
}

pub fn parseInput(allocator: std.mem.Allocator, data: []const u8) MachineState {
    var lineIter = std.mem.splitScalar(u8, data, '\n');
    const a = parseReg(lineIter.next().?);
    const b = parseReg(lineIter.next().?);
    const c = parseReg(lineIter.next().?);
    _ = lineIter.next();
    var programIter = std.mem.splitScalar(u8, lineIter.next().?, ':');
    _ = programIter.next();
    const programStr = std.mem.trim(u8, programIter.next().?, &[4]u8{ ' ', '\n', '\t', '\r' });

    var instrs = std.ArrayList(u3).init(allocator);

    var instrIter = std.mem.splitScalar(u8, programStr, ',');
    while (instrIter.next()) |instStr| {
        const value = std.fmt.parseInt(u3, instStr, 0) catch {
            @panic("Unable to parse instr");
        };
        instrs.append(value) catch {
            @panic("Out of memory");
        };
    }

    const out = std.ArrayList(usize).init(allocator);
    return MachineState{
        .a = a,
        .b = b,
        .c = c,
        .pc = 0,
        .instrs = instrs,
        .out = out,
    };
}

const Opcode = enum(u3) {
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7,
};

pub fn parseComboOperand(state: *MachineState) usize {
    const rawOperand: u3 = state.instrs.items[state.pc];
    state.pc += 1;

    return switch (rawOperand) {
        4 => state.a,
        5 => state.b,
        6 => state.c,
        7 => {
            @panic("Invalid combo operand");
        },
        else => rawOperand,
    };
}

pub fn parseLiteralOperand(state: *MachineState) usize {
    const rawOperand: u3 = state.instrs.items[state.pc];
    state.pc += 1;
    return rawOperand;
}

pub fn pow(base: usize, exp: usize) usize {
    var result: usize = 1;
    for (0..exp) |_| {
        result *= base;
    }
    return result;
}

pub fn stepInstr(state: *MachineState) bool {
    if (state.pc >= state.instrs.items.len) {
        return false;
    }

    const inst: Opcode = @enumFromInt(state.instrs.items[state.pc]);
    std.log.debug("Instr: {}, pc {}", .{ inst, state.pc });
    state.pc += 1;

    switch (inst) {
        .adv => {
            const operand = parseComboOperand(state);
            state.a = state.a / pow(2, operand);
        },
        .bxl => {
            const operand = parseLiteralOperand(state);
            state.b = state.b ^ operand;
        },
        .bst => {
            const operand = parseComboOperand(state);
            state.b = operand % 8;
        },
        .jnz => {
            const operand = parseLiteralOperand(state);
            if (state.a != 0) {
                state.pc = operand;
            }
        },
        .bxc => {
            _ = parseLiteralOperand(state);
            state.b = state.b ^ state.c;
        },
        .out => {
            const operand = parseComboOperand(state);
            state.out.append(operand % 8) catch {
                @panic("could not append output");
            };
        },
        .bdv => {
            const operand = parseComboOperand(state);
            state.b = state.a / pow(2, operand);
        },
        .cdv => {
            const operand = parseComboOperand(state);
            state.c = state.a / pow(2, operand);
        },
    }
    return true;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer {
        _ = gpa.deinit();
    }
    const allocator = gpa.allocator();
    const data = try std.fs.cwd().readFileAlloc(allocator, "./realinput.txt", std.math.maxInt(usize));
    defer allocator.free(data);

    var state = parseInput(allocator, data);
    defer state.deinit();

    std.log.debug("Initial state", .{});
    std.log.debug("reg a {}", .{state.a});
    std.log.debug("reg b {}", .{state.b});
    std.log.debug("reg c {}", .{state.c});
    for (state.instrs.items) |inst| {
        std.log.debug("{}", .{inst});
    }
    var step: usize = 1;
    while (stepInstr(&state)) {
        std.log.debug("Step {}", .{step});
        std.log.debug("reg a {}", .{state.a});
        std.log.debug("reg b {}", .{state.b});
        std.log.debug("reg c {}", .{state.c});
        step += 1;
    }

    for (state.out.items, 0..) |byte, idx| {
        std.debug.print("{}", .{byte});
        if (idx != state.out.items.len - 1) {
            std.debug.print(",", .{});
        }
    }
}
