include("./src/Simple.jl")

using .Simple

function power_of_two_st(n::Int)
    Statement[
        Assign(:c, Num(0)),
        Assign(:r, Num(1)),
        Simple.While(
            LessThan(Variable(:c), Num(n)),
            Sequence(
                Assign(:r, Mul(Variable(:r), Num(2))),
                Assign(:c, Add(Variable(:c), Num(1)))
            )
        )
    ] |> join_statements
end

function fibonacci(n::Int)
    body = Statement[
        Assign(:tmp, Variable(:a)),
        Assign(:a, Variable(:b)),
        Assign(:b, Add(Variable(:tmp), Variable(:b))),
        Assign(:c, Add(Variable(:c), Num(1)))
    ] |> join_statements
    Statement[
        Assign(:a, Num(0)),
        Assign(:b, Num(1)),
        Assign(:c, Num(0)),
        Simple.While(
            LessThan(Variable(:c), Num(n)),
            body
        )
    ] |> join_statements
end

function main()
    m = power_of_two_st(10) |> Machine
    mrun(m)
    println("SmallStep: $(m.environment[:r])")

    s = power_of_two_st(10)
    println("BigStep: $(Simple.evaluate(s)[:r])")

    fm = fibonacci(10) |> Machine
    mrun(fm)
    println("SmallStep: $(fm.environment[:b])")

    fs = fibonacci(10)
    println("BigStep: $(Simple.evaluate(fs)[:b])")
end

main()
