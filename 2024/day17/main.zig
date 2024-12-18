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

    fn clone(self: *const MachineState) !MachineState {
        return MachineState{ .a = self.a, .b = self.b, .c = self.c, .pc = self.pc, .instrs = try self.instrs.clone(), .out = try self.out.clone() };
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

const ProgramErr = error{
    invalid_combo_operand,
};

pub fn parseComboOperand(state: *MachineState) !usize {
    const rawOperand: u3 = state.instrs.items[state.pc];
    state.pc += 1;

    return switch (rawOperand) {
        4 => state.a,
        5 => state.b,
        6 => state.c,
        7 => ProgramErr.invalid_combo_operand,
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

pub fn stepInstr(state: *MachineState) !bool {
    if (state.pc >= state.instrs.items.len) {
        return false;
    }

    const inst: Opcode = @enumFromInt(state.instrs.items[state.pc]);
    state.pc += 1;

    switch (inst) {
        .adv => {
            const operand = try parseComboOperand(state);
            state.a = state.a / pow(2, operand);
        },
        .bxl => {
            const operand = parseLiteralOperand(state);
            state.b = state.b ^ operand;
        },
        .bst => {
            const operand = try parseComboOperand(state);
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
            const operand = try parseComboOperand(state);
            state.out.append(operand % 8) catch {
                @panic("could not append output");
            };
        },
        .bdv => {
            const operand = try parseComboOperand(state);
            state.b = state.a / pow(2, operand);
        },
        .cdv => {
            const operand = try parseComboOperand(state);
            state.c = state.a / pow(2, operand);
        },
    }
    return true;
}

pub fn tryProgram(machine: *const MachineState, reg_a: usize) std.ArrayList(usize) {
    var state = machine.clone() catch {
        @panic("Out of memory");
    };
    defer state.deinit();

    state.a = reg_a;

    var step: usize = 1;
    while (stepInstr(&state) catch {
        @panic("unhandled instruction");
    }) {
        step += 1;
    }

    return state.out.clone() catch {
        @panic("Out of memory");
    };
}

fn solve_a(part_a: usize, target_idx: usize, state: *const MachineState) ?usize {
    for (0..8) |option| {
        const result = tryProgram(state, part_a * 8 + option);
        defer result.deinit();

        if (result.items[0] == state.instrs.items[target_idx]) {
            if (target_idx == 0) {
                return part_a * 8 + option;
            }

            if (solve_a(part_a * 8 + option, target_idx - 1, state)) |solution| {
                return solution;
            }
        }
    }

    return null;
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

    if (solve_a(0, state.instrs.items.len - 1, &state)) |a| {
        const list = tryProgram(&state, a);
        defer list.deinit();
        for (list.items, state.instrs.items, 0..) |got, exp, idx| {
            if (got != exp) {
                std.debug.print("Results do not match at idx {}: {} vs {}!\n", .{ idx, got, exp });
                return;
            }
        }
        std.debug.print("solution {}\n", .{a});
    }
}
