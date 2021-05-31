include("./src/expression.jl")
include("./src/statement.jl")

using Test

@testset "all" begin

@testset "to_s" begin
    @test to_s(Num(3)) == "3"
    @test to_s(Add(Num(3), Num(4))) == "3 + 4"
    @test to_s(Mul(Num(1), Num(3))) == "1 * 3"
    @test to_s(Boolean(false)) == "false"
    @test to_s(LessThan(Num(3), Num(5))) == "3 < 5"
    @test to_s(Add(Num(3), Mul(Num(1), Num(2)))) == "3 + 1 * 2"
end

@testset "inspect" begin
    @test inspect(Num(3)) == "<< 3 >>"
    @test inspect(Add(Num(1), Num(2))) == "<< 1 + 2 >>"
    @test inspect(Mul(Num(1), Num(2))) == "<< 1 * 2 >>"
    @test inspect(Boolean(false)) == "<< false >>"
    @test inspect(LessThan(Num(3), Num(5))) == "<< 3 < 5 >>"
    @test inspect(Add(Num(1), Mul(Num(2), Num(3)))) == "<< 1 + 2 * 3 >>"
end

@testset "reducible" begin
    @test !reducible(Num(3))
    @test reducible(Add(Num(1), Num(3)))
    @test reducible(Mul(Num(2), Num(2)))
    @test !reducible(Boolean(false))
    @test reducible(LessThan(Num(1), Num(5)))
end

@testset "reduce" begin
    @test reduce(Add(Num(3), Num(4)), Environment()) == Num(7)
    @test reduce(Mul(Num(2), Num(3)), Environment()) == Num(6)
end

@testset "variable" begin
    @test to_s(Variable(:x)) == "x"
    @test inspect(Variable(:x)) == "<< x >>"
    @test reducible(Variable(:x))
    @test reduce(Variable(:x), Environment(:x => Num(4))) == Num(4)
end

@testset "statement" begin
    @testset "to_s" begin
        @test to_s(DoNothing()) == "do-nothing"
        @test to_s(Assign(:x, Num(3))) == "x = 3"
    end
    @testset "inspcect" begin
        @test inspect(DoNothing()) == "<< do-nothing >>"
        @test inspect(Assign(:x, Num(3))) == "<< x = 3 >>"
    end
    @testset "reducible" begin
        @test !reducible(DoNothing())
        @test reducible(Assign(:x, Num(3)))
    end
    @testset "reduce" begin
        s, env = reduce(Assign(:x, Num(3)), Environment())
        @test s == DoNothing()
        @test env == Dict(:x => Num(3))
    end
end

@testset "If" begin
    if_s = If(LessThan(Num(3), Num(4)), Assign(:x, Num(10)), Assign(:x, Num(100)))
    @test to_s(if_s) == "If 3 < 4 { x = 10 } else { x = 100 }"
    @test inspect(if_s) == "<< If 3 < 4 { x = 10 } else { x = 100 } >>"
    @test reducible(if_s)
    @test reduce(if_s, Environment()) == (If(Boolean(true), Assign(:x, Num(10)), Assign(:x, Num(100))), Environment())
end

@testset "While" begin
    while_s = While(
        LessThan(Variable(:x), Num(10)),
        Assign(:y, Num(20))
    )
    @test to_s(while_s) == "While { x < 10 } { y = 20 }"
    @test reducible(while_s)
    if_s = If(
        LessThan(Variable(:x), Num(10)),
        Sequence(Assign(:y, Num(20)), while_s),
        DoNothing()
    )
    s, env = reduce(while_s, Environment())
    @test s == if_s
    @test env == Environment()
end

@testset "Sequence" begin
    seq = Sequence(Assign(:x, Num(1)), Assign(:y, Num(2)))
    @test to_s(seq) == "x = 1; y = 2"
    @test inspect(seq) == "<< x = 1; y = 2 >>"
    s, env = reduce(seq, Environment())
    @test s == Sequence(DoNothing(), Assign(:y, Num(2)))
    @test env == Environment(:x => Num(1))
end

@testset "join_statements" begin
    ss = Statement[
        Assign(:x, Add(1, 10)),
        Assign(:y, Add(Variable(:x), Num(100))),
        Assign(:z, Add(Variable(:y), Num(1000)))
    ]
    @test join_statements(ss) == Sequence(
        Assign(:x, Add(1, 10)),
        Sequence(
            Assign(:y, Add(Variable(:x), Num(100))),
            Assign(:z, Add(Variable(:y), Num(1000)))
        )
    )
end

@testset "machine" begin
    @testset "step" begin
        m = Machine(Assign(:x, Add(1, 2)))
        step(m)
        s, env = reduce(Assign(:x, Add(1, 2)), Environment())
        @test m.statement == s
        @test m.environment == env
    end
    @testset "run" begin
        m = Machine(
            If(LessThan(Num(1), Num(2)), Assign(:x, Num(99)), Assign(:x, Num(999)))
        )
        run(m)
        @test m.statement == DoNothing()
        @test m.environment == Environment(:x => Num(99))
        m2 = Machine(
            If(LessThan(Num(10), Num(2)), Assign(:x, Num(99)), Assign(:x, Num(999)))
        )
        run(m2)
        @test m2.environment == Environment(:x => Num(999))
    end
    @testset "while run" begin
        s = Statement[
            Assign(:x, Num(1)),
            While(
                LessThan(Variable(:x), Num(5)),
                Assign(:x, Mul(Variable(:x), Num(3)))
            )
        ] |> join_statements
        m = Machine(s)
        run(m)
        @test m.environment == Environment(:x => Num(9))
    end
    @testset "sequence run" begin
        m = Machine(
            Sequence(Assign(:x, Add(1, 2)), Assign(:y, Add(Variable(:x), Num(3))))
        )
        run(m)
        @test m.environment == Environment(:x => Num(3), :y => Num(6))
    end
end

@testset "evaluate" begin
    @testset "Expression" begin
        @test evaluate(Num(3), Environment()) == Num(3)
        @test evaluate(Boolean(false), Environment()) == Boolean(false)
        @test evaluate(Variable(:x), Environment(:x => Num(3))) == Num(3)
        @test evaluate(Add(Num(3), Num(5)), Environment()) == Num(8)
        @test evaluate(Mul(Num(2), Num(3)), Environment()) == Num(6)
        @test evaluate(LessThan(Num(3), Num(10)), Environment()) == Boolean(true)
    end
    @testset "Nested Expression" begin
        @test evaluate(Add(Variable(:x), Num(3)), Environment(:x => Num(10))) == Num(13)
    end
    @testset "Statement" begin
        @test evaluate(DoNothing(), Environment(:x => Num(1))) == Environment(:x => Num(1))
        @test evaluate(
            If(
               LessThan(Num(1), Num(2)),
               Assign(:x, Num(1)),
               Assign(:y, Num(11))),
            Environment()) == Environment(:x => Num(1))
        @test evaluate(Sequence(Assign(:x, Num(1)), Assign(:y, Num(11))), Environment()) == Environment(:x => Num(1), :y => Num(11))
        @test evaluate(Sequence(Assign(:x, Num(1)), Assign(:x, Num(11))), Environment()) == Environment(:x => Num(11))
    end
    @testset "While" begin
        while_s = While(
            LessThan(Variable(:x), Num(5)),
            Assign(:x, Mul(Variable(:x), Num(3)))
        )
        @test evaluate(while_s, Environment(:x => Num(1))) == Environment(:x => Num(9))
    end
end

end
