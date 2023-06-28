local COMMANDS = {}

local file = io.open('./day_2_input.txt', 'r')

if file then
  for line in file:lines() do
    print(line)
    table.insert(COMMANDS, line)
  end
  file:close()
end

local function part_1(commands)
  local horizontal_position = 0
  local depth = 0


  for _, command_string in ipairs(commands) do
    for command, amount in command_string:gmatch('(%w+) (%d+)') do
      if command == "forward" then
        horizontal_position = horizontal_position + amount
      end
      if command == "down" then
        depth = depth + amount
      end
      if command == "up" then
        depth = depth - amount
      end
    end
  end
  return horizontal_position * depth
end

local function part_2(commands)
  local horizontal_position = 0
  local aim = 0
  local depth = 0

  for _, command_string in ipairs(commands) do
    for command, amount in command_string:gmatch('(%w+) (%d+)') do
      if command == "forward" then
        horizontal_position = horizontal_position + amount
        depth = depth + aim * amount
      end
      if command == "down" then
        aim = aim + amount
      end
      if command == "up" then
        aim = aim - amount
      end
    end
  end
  return horizontal_position * depth
end

print("part 1: " .. part_1(COMMANDS))
print("part 2: " .. part_2(COMMANDS))
