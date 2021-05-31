abstract type Expression end

Environment = Dict{Symbol, Expression}

struct Num <: Expression
    value::Int
end

struct Add <: Expression
    left::Expression
    right::Expression
end

Add(l::Int, r::Int) = Add(Num(l), Num(r))

struct Mul <: Expression
    left::Expression
    right::Expression
end

Mul(l::Int, r::Int) = Mul(Num(l), Num(r))

struct Boolean <: Expression
    value::Bool
end

struct LessThan <: Expression
    left::Expression
    right::Expression
end

to_s(exp::Num) = string(exp.value)
to_s(exp::Add) = to_s(exp.left) * " + " * to_s(exp.right)
to_s(exp::Mul) = to_s(exp.left) * " * " * to_s(exp.right)
to_s(exp::Boolean) = string(exp.value)
to_s(exp::LessThan) = to_s(exp.left) * " < " * to_s(exp.right)

reducible(::Num) = false
reducible(::Boolean) = false
reducible(::Expression) = true

inspect(exp::Expression) = "<< " * to_s(exp) * " >>"

function reduce(exp::Add, env::Environment)::Expression
    return if reducible(exp.left)
        Add(reduce(exp.left, env), exp.right)
    elseif reducible(exp.right)
        Add(exp.left, reduce(exp.right, env))
    else
        Num(exp.left.value + exp.right.value)
    end
end

function reduce(exp::Mul, env::Environment)::Expression
    return if reducible(exp.left)
        Mul(reduce(exp.left, env), exp.right)
    elseif reducible(exp.right)
        Mul(exp.left, reduce(exp.right, env))
    else
        Num(exp.left.value * exp.right.value)
    end
end

function reduce(exp::LessThan, env::Environment)::Expression
    return if reducible(exp.left)
        LessThan(reduce(exp.left, env), exp.right)
    elseif reducible(exp.right)
        LessThan(exp.left, reduce(exp.right, env))
    else
        Boolean(exp.left.value < exp.right.value)
    end
end

struct Variable <: Expression
    name::Symbol
end

to_s(v::Variable) = String(v.name)

function reduce(v::Variable, env::Environment)::Expression
    return env[v.name]
end

evaluate(exp::Num, ::Environment) = exp
evaluate(exp::Boolean, ::Environment) = exp
evaluate(exp::Variable, env::Environment) = env[exp.name]
evaluate(exp::Add, env::Environment) = (evaluate(exp.left, env).value + evaluate(exp.right, env).value) |> Num
evaluate(exp::Mul, env::Environment) = (evaluate(exp.left, env).value * evaluate(exp.right, env).value) |> Num
evaluate(exp::LessThan, env::Environment) = (evaluate(exp.left, env).value < evaluate(exp.right, env).value) |> Boolean

