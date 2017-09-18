# JSON Interface

This document describes the interface between the frontend and the
backend.  This should allow for a more mechanical development process
as well as targets for multiple frontends and multiple backends to
allow experimenting with various frameworks, tools, and designs.

NOTE: This interface will likely have to change before this becomes
useful a "product" worth using; things like multi-media files are
probably essential.


## Authentication

In a multi-user environment (what the design has been up to this
point), the user must authenticate.  The user will enter a password or
otherwise have a token be sent to the server.

The server will expect:

```JSON
{
    username: String,
    password: String,
}
```

TODO: should the password be hashed or otherwise destructively
processed on the client-side?  Probably not, but verify and update
this when future-me has figured this out. (I assume this always
results in less security (hash collisions, fewer bits of info...) and
if the password being sent is insecure, then any processed password
would be easily abusable unless a public-private key set-up could
happen, but even then, the lack of security suggests that wouldn't
even be feasible)


The Server will respond with:

```JSON
{
    auth_token: String,
}
```

TODO: verify this


## Article submission

When the user finds a new article or otherwise triggers the submission
of an article from the frontend, the frontend must send:

```JSON
{
    auth_token: String,
    title: String,
    content: String,
    language: String,
}
```


The server will then respond with:

```JSON
{
    result: null or "Success",
}
```

A result of `null` means that the language was not recognized.  Any
other response but `"Success"` means that there was a server error,
possibly the client send content length outside that which will be
accepted by the server.  This will be discussed/documented later.


## Article retrieval

When the user wishes to interact with an article, they must send:

```JSON
{
    auth_token: String,
    title: String,
    language: String,
}
```


The server will respond with:

```JSON
{
    title: String,
    content_in_words: [String],
    unknown_words: [{String: Language-defined-JSON}],
    studying_words: [(String, Int)],
    language: String,
}
```

The `unknown_words` field contains an array of key value pairs where
the keys are words that user has not indicated knowledge of and the
values are JSON objects that are defined by the language.  Thus, it
will likely include fields such as "definition" and may contain fields
such as "pronunciation" or other things which are not necessarily
universal between languages.

The `studying_words` field indicates knowledge level as defined by the
SRS algorithm.


Alternatively, if the request article isn't found, the server will
respond with:

```JSON
null
```


### Selecting a word

When the user selects a word, the frontend will send:

```JSON
{
    auth_token: String,
    word: String,
    knowledge_level: Int,
}
```


## Review

When the user opts for explicit review, a flash-card like interface
will be presented.  The frontend will send:

```JSON
{
    language: String,
}
```

TODO: possibly include metadata here


The server will respond with:

```JSON
{
    language: String,
    words_to_review: [{String: language-defined-JSON}],
}
```

See previous section for information about `language-defined-JSON`.


Alternatively, if the user-specified language does not exist the
server will respond with:

```JSON
null
```

However, if the language does exist but there are no words to review
(either because they've all been reviewed recently or the user has no
"known words", then the server will respond with:

```JSON
"Nothing to do"
```
