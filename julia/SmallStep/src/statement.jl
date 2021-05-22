abstract type Statement end

struct DoNothing <: Statement end

struct Assign <: Statement
    name::Symbol
    expression::Expression
end

struct If <: Statement
    condition::Expression
    consequence::Statement
    alternative::Statement
end

struct While <: Statement
    condition::Expression
    body::Statement
end

struct Sequence <: Statement
    first::Statement
    second::Statement
end

function join_statements(ss::AbstractArray{Statement})
    if length(ss) == 1
        return ss[1]
    else
        return Sequence(ss[1], join_statements(ss[2:end]))
    end
end

to_s(::DoNothing) = "do-nothing"
to_s(s::Assign) = string(s.name) * " = " * to_s(s.expression)
to_s(s::If) = "If " * to_s(s.condition) * " { " * to_s(s.consequence) * " } else { " * to_s(s.alternative) * " }"
to_s(s::Sequence) = to_s(s.first) * "; " * to_s(s.second)
to_s(s::While) = "While { " * to_s(s.condition) * " } { " * to_s(s.body) * " }"

inspect(s::Statement) = "<< " * to_s(s) * " >>"
reducible(::DoNothing) = false
reducible(::Statement) = true

function reduce(s::Assign, env::Environment)::Tuple{Statement, Environment}
    if s.expression |> reducible
        (Assign(s.name, reduce(s.expression, env)), env)
    else
        (DoNothing(), merge(env, Dict(s.name => s.expression)))
    end
end

function reduce(s::If, env::Environment)::Tuple{Statement, Environment}
    if s.condition |> reducible
        (If(reduce(s.condition, env), s.consequence, s.alternative), env)
    else
        if s.condition == Boolean(true)
            (s.consequence, env)
        elseif s.condition == Boolean(false)
            (s.alternative, env)
        else
            error("condition must be a boolean")
        end
    end
end

function reduce(s::While, env::Environment)::Tuple{Statement, Environment}
    (If(s.condition, Sequence(s.body, s), DoNothing()), env)
end

function reduce(s::Sequence, env::Environment)::Tuple{Statement, Environment}
    if s.first == DoNothing()
        sec, env = reduce(s.second, env)
        (sec, env)
    else
        f, env = reduce(s.first, env)
        (Sequence(f, s.second), env)
    end
end

mutable struct Machine
    statement::Statement
    environment::Environment
end

Machine(s::Statement) = Machine(s, Environment())

function step(m::Machine)
    s, env = reduce(m.statement, m.environment)
    m.statement = s
    m.environment = env
end

function run(m::Machine)
    while reducible(m.statement)
        step(m)
    end
end
