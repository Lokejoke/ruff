# Example 1: Multiplying a list of mutable strings
row = [""] * 3  # Creates a list with 3 references to the same empty string
tic_tac_toe = [row] * 3  # This will create 3 references to the same `row` list
tic_tac_toe[0][0] = 'X'  # This affects all rows because they refer to the same `row` list

# Example 2: Multiplying a list of mutable dictionaries
board = [{"cell": ""}] * 3  # Creates a list with 3 references to the same dictionary
tic_tac_toe_dict = [board] * 3  # This will create 3 references to the same `board` list
tic_tac_toe_dict[0][0]["cell"] = 'X'  # This affects all dictionaries because they refer to the same object

# Example 3: Multiplying a list of sets
set_a = set()  # Create an empty set
set_list = [set_a] * 3  # Creates a list with 3 references to the same set
set_list[0].add(1)  # This affects all sets because they refer to the same set

# Example 4: Nested lists of mutable objects (list of lists)
nested_board = [[""] * 3 for _ in range(3)]  # This creates separate lists for each row
tic_tac_toe_nested = [nested_board] * 3  # This will create 3 references to the same `nested_board` list
tic_tac_toe_nested[0][0][0] = 'X'  # This affects all nested lists because they refer to the same object

# Example 5: Correct approach using list comprehension (no violation)
tic_tac_toe_fixed = [[''] * 3 for _ in range(3)]  # Creates separate lists for each row
tic_tac_toe_fixed[0][0] = 'X'  # Only the first row should be affected
