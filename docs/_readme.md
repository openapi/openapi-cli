# HTTP Clients, APIs, and a Small Easter Egg

An HTTP client is a program or library that sends requests to a server and reads the responses. In practice, it is the part of a system that knows how to call remote endpoints, pass headers, send payloads, and interpret status codes and returned data.

REST APIs and RPC APIs both let one system talk to another, but they express that conversation differently. REST is usually organized around resources such as users, invoices, or documents, and it uses HTTP verbs like `GET`, `POST`, `PUT`, and `DELETE` to describe the action. RPC is organized around actions or procedures, so the request is more like "run this operation" than "interact with this resource". REST tends to feel more resource-oriented and uniform, while RPC tends to feel more direct and operation-oriented.

## The Historical Meaning of "Plugh"

"Plugh" is a small but famous piece of computing folklore. It is best known as a magic word from early text adventures, especially the Colossal Cave tradition, where words like `xyzzy` and `plugh` worked as hidden triggers, teleports, or secret incantations. Over time, the term survived as a kind of hacker shibboleth: short, strange, playful, and immediately recognizable to people who enjoy the older layers of software culture.

That is why the word still works well as an easter egg. It sounds technical without being descriptive, and historical without being solemn. It belongs to the long lineage of inside jokes that moved from terminals to source trees, from source trees to changelogs, and from changelogs to commit messages.

And if `Plugh` feels oddly familiar here, maybe this is not the first history where it was left behind on purpose.
