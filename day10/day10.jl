# https://juliabyexample.helpmanual.io/
# https://docs.julialang.org/en/v1/manual/getting-started/

using Pipe: @pipe

function neighbors(p::CartesianIndex{2})
    p .+ [CartesianIndex(n...) for n in [(1,0), (-1,0), (0,1), (0,-1)]]
end

function withinBounds(p::CartesianIndex{2}, grid::Matrix{Int})
    rows, cols = size(grid)
    row, col = Tuple(p)
    1 <= row <= rows && 1 <= col <= cols
end

function getScore(p::CartesianIndex{2}, grid::Matrix{Int}) :: Set{CartesianIndex{2}}
    current = grid[p]
    if current == 9
        return Set([p])
    end
    N = @pipe neighbors(p) |> filter(n -> withinBounds(n, grid) && grid[n] == current + 1, _)
    s = Set()
    for n in N
        s = union(s, getScore(n, grid))
    end
    s
end

function getScore2(p::CartesianIndex{2}, grid::Matrix{Int}) :: Int
    current = grid[p]
    if current == 9
        return 1
    end
    N = @pipe neighbors(p) |> filter(n -> withinBounds(n, grid) && grid[n] == current + 1, _)
    s = 0
    for n in N
        s += getScore2(n, grid)
    end
    s
end

input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
input = read(open("input.txt","r"), String)

parseline(line) = @pipe line |> split(_, "") |> map(c -> parse(Int, c), _)

lines = @pipe input |> split(_, "\n")
G = @pipe lines |> map(parseline, _) |> hcat(_...)

println(sum([length(getScore(p, G)) for p in findall(v -> v==0, G)]))
println(sum([getScore2(p, G) for p in findall(v -> v==0, G)]))
