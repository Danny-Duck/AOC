local sample_input = { 199, 200, 208, 210, 200, 207, 240, 269, 260, 263 }

local file = io.open("./day_1_input.txt", "r")
local depths = {}
local increased_count_part_1 = 0

if file then
  for depth in file:lines() do
    table.insert(depths, depth)
  end
  file:close()
end


for i, current in ipairs(depths) do
  local next = depths[i + 1]

  if next then
    print(next .. " > " .. current, next > current, type(current), type(next))
    if next > current then
      increased_count_part_1 = increased_count_part_1 + 1
    end
  else
    print(current)
  end
end

print("part 1", increased_count_part_1)

local increased_count_part_2 = 0

for i = 0, #depths do
  if depths[i] and depths[i + 1] and depths[i + 2] and depths[i + 3] then
    local current_window = depths[i] + depths[i + 1] + depths[i + 2]
    local next_window = depths[i + 1] + depths[i + 2] + depths[i + 3]

    if next_window > current_window then
      increased_count_part_2 = increased_count_part_2 + 1
    end
    --[[ else ]]
    --[[ print(current) ]]
  end
end

print("part 2", increased_count_part_2)
