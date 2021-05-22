include("./src/Compute.jl")

using .Compute

function main()
    s = Statement[
        Assign(:c, Num(0)),
        Assign(:r, Num(1)),
        Compute.While(
            LessThan(Variable(:c), Num(10)),
            Sequence(
                Assign(:r, Mul(Variable(:r), Num(2))),
                Assign(:c, Add(Variable(:c), Num(1)))
            )
        )
    ] |> join_statements
    m = Machine(s)
    println("$(to_s(s))")
    mrun(m)
    println(m.environment)
end

main()
