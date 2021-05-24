module SmallStep

include("./expression.jl")
include("./statement.jl")

export Machine
export Num
export Add
export Mul
export Variable
export Boolean
export LessThan
export If
export Sequence
export Assign
export Statement

mrun(m::Machine) = run(m::Machine)

export mrun
export join_statements
export inspect
export to_s

end
