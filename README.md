# calendaria 

### What is it?

It'll just be a tool for me to intuitively organize/schedule out my calendar. Maybe I'll extend it for people to use, maybe not. ¯\\\_(ツ)_/¯

### What store is being used?

I'm hoping to leverage a no-store infrastructure. No DB, no document store, just some files that build with script (unless I need to be fancy later), but I want to only leverage google calendar.

### Thoughts

I want to keep this as cheap as possible. I'll be pushing this into AWS and be keeping usage under free tier. I want this to just be a simple Lambda that will be kicked off x amount of times a day that will organize my calendar.


### TODO
- [] Base Populate Google Calendar (I'll need to manually block off things on my calendar. M-F have work 7:30 - 3:30)
- [] Create struct of event types (manual vs automated), priority, estimated time.
- [] Create algo for next best time (start simple by looking at current day and next day. But can get more sophisticated later?)
- [] Create Syncing logic (store in Google Calendar description)