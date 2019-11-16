Hash map that maps interactable_name to interactable_id
X move logic: the logic will be if exit is not locked then go through the exit

update to interact logic:
    if interactable has a prerequisite item:
        you currently can not interact with X.  Find more items and try again.

unlock logic
use item_name on exit_name
Loop through exits to find one with the appropriate name.
if the door is locked:
    loop through interactables and find the one with the door's interactable_id name
    if the interactable's prerequisite item is the item you used, interact
    unlock the exit
    return the interaction text

if exit is locked and exit.


