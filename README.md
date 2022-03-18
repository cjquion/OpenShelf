features
--

sharing:
 - share your collections with your friends!
 - comment on your favorite parts of the song! 
 - leave messages in the playlist

organization:
 - tag your songs/playlists (rating, genre...)
 - playlist folders
 - listening statistics / last-fm scrobbling?
 - grouping (album/artist/tag/chronology/etc)
 - add song-specific pictures

standard features:
 - playback queue/order
 - search by artist/album/playlist/tag
 - vault organization (remove dupes, search for album art...)
 - lyrics!

usage 
--
Establishes a local directory to act as a vault for OpenShelf.
```
openshelf register-vault --local <<path>> name
```

Establishes a remote vault 
```
openshelf register-vault --remote <<host>><<path>> name
```

Syncs your files 
```
openshelf sync-vault <<vaultname>>
```

```
openshelp play <<path>>
```

Queue up a song.
```
openshelf q <<>>
```

