local playlists = redis.call('KEYS', 'playlist:*')

for i,playlist_key in ipairs(playlists) do
    local members = {}
    local needs_update = false
    for i in string.gmatch(redis.call('HGET', playlist_key, 'members'), '%S+') do
        if i == ARGV[1] then
            needs_update = true
        else
            table.insert(members, i)
        end
    end
    if needs_update then
        redis.call('HSET', playlist_key, 'members', table.concat(members, ' '))
    end
end

local lyric_key = table.concat({'lyric', ARGV[1]}, ':')
redis.call('DEL', lyric_key)

return
