local lyrics = [[
Verse 1

Hope for tomorrow
On the edge of despair
Faith in the unknown
And you'll make it there

Chorus

Believe in something
And never give up
Hope for tomorrow
And it'll fill your cup

Verse 2

There's a light ahead
In the darkest sky
You can be yourself
And you can live the life

Chorus

Believe in something
And never give up
Hope for tomorrow
And it'll fill your cup

Verse 3

We can make a change
If we act now
Be the one that stands out
And watch the stars align

Chorus

Believe in something
And never give up
Hope for tomorrow
And it'll fill your cup

Bridge

You can make it, if you try
Your dreams will come alive
So never forget

Chorus

Believe in something
And never give up
Hope for tomorrow
And it'll fill your cup

Outro

Hope for tomorrow
And never give up
Believe in something 
And it will fill your cup]]

function WriteLyricsToFile(filename)
  local file = io.open(filename, "w")
  file:write(lyrics)
  file:close()
end

WriteLyricsToFile("hope_for_tomorrow.txt")