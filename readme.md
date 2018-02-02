
# Using an Async Hyper Client 使用异步 Hyper Client

> Published March 9th, 2017 by [Michael Gattozzi](https://mgattozzi.com/hyper-client) <br/>
> [Github 中文翻译链接 https://github.com/aqrun/request_demo/blob/master/readme.md](https://github.com/aqrun/request_demo/blob/master/readme.md) (翻译的不好，任何翻译问题欢迎指正)

Lately I've been revamping my [GitHub API Library](https://github.com/mgattozzi/github-rs) to be both more ergonomic and to use the upcoming 0.11 release of [Hyper](https://github.com/hyperium/hyper) which is asynchronous using [Futures](https://github.com/alexcrichton/futures-rs) and [Tokio](https://tokio.rs/) under the hood. Mainly this has been due to my experiences using my library in my GitHub bot [Thearesia](https://github.com/mgattozzi/thearesia). I figured if I'm already going to be redoing how my library works might as well upgrade to the new version of Hyper as well and provide some explanations to those wishing to upgrade their own libraries. I'll be using Hyper at [this commit](https://github.com/hyperium/hyper/tree/e871411627cab5caf00d8ee65328da9ff05fc53d) for today's example. The docs are good enough for now if you want to dig into it, but you might need to fish around for what you need. Good news is this seems to be the [last issue open](https://github.com/hyperium/hyper/issues/805) before release!

最近，我一直在修改我的[GitHub API库](https://github.com/mgattozzi/github-rs)，使其更符合人体工程学，并使用即将推出的0.11版本的[Hyper](https://github.com/hyperium/hyper)，它的底层是[Futures](https://github.com/alexcrichton/futures-rs)和[Tokio](https://tokio.rs/)的异步应用。主要是基于我在我的GitHub bot [Thearesia](https://github.com/mgattozzi/thearesia)中使用我的库的经验。我想如果我升级到Hyper新版本我的库也可以正常运行，并且为那些希望升级他们自己的库的人们提供一些解释。在今天的例子中，我将在[这个提交](https://github.com/hyperium/hyper/tree/e871411627cab5caf00d8ee65328da9ff05fc53d)中使用Hyper。如果你想深入了解，现在的文档已经足够好了，但是你可能需要为你所需要的东西进一步探索。好消息是这似乎是发布前[最后一个issue](https://github.com/hyperium/hyper/issues/805)！

Before we begin I'm assuming you have a cursory knowledge of Futures and Tokio. If you need an introduction to it I'd highly recommend reading Andrew Hobden's post over on Asquera's blog called [The Future with Futures](http://asquera.de/blog/2017-03-01/the-future-with-futures/). It's an informative read and should cover enough of what we need to know for this example!

在开始之前，我假设你对Futures和Tokio有粗略的了解。 如果你需要了解，我强烈推荐阅读Andrew Hobden发布在Asquera的博客[The Future with Futures](http://asquera.de/blog/2017-03-01/the-future-with-futures/)。 这是一个内容丰富的阅读，涵盖了我们需要知道的这个例子足够信息！

Today, we'll go through making a request to the GitHub API asking for ourselves as a user (in this case I'll be making a request using Thearesia's token but follow along using [your own](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/)). This means we'll need HTTPS support so we'll be importing the hyper-tls library as well. This is a more involved example than what is in the Hyper repo currently and should help cover a good few use cases for people.

今天，我们将向GitHub API发出一个请求，要求我们自己作为一个用户（在这种情况下，我将使用Thearesia的token发出请求，但后面的例子请使用[您自己](https://help.github.com/articles/creating-a-personal-access-token-for-the-command-line/)的token）。 这意味着我们需要HTTPS支持，所以我们也会导入hyper-tls库。 这是一个比Hyper库目前更为复杂的例子，应该包含很多的使用案例。

Let's get started by creating a new project:

让我们开始创建一个新的项目：

```shell
cargo new --bin ghub
```

Then open up our new Cargo.toml file and add these lines:

然后打开Cargo.toml添加下面的依赖

```shell
hyper = { git = "https://github.com/hyperium/hyper" }
hyper-tls = { git = "https://github.com/hyperium/hyper-tls" }
tokio-core = "0.1"
futures = "0.1"
```

This will give use the newest version of hyper and hyper-tls since it'll be using the git dependency (if you're from the future and following along try 0.11 as the version to use instead if it's out and the examples below are failing). Your Cargo.toml should look something like this now:

这将使用最新版本的hyper和hyper-tls，因为它将使用git依赖（将来会继续使用0.11版本，而不会出了新版本，下面的例子执行失败）。 你的Cargo.toml应该看起来像这样：

```toml
[package]
name = "ghub"
version = "0.1.0"
authors = ["Michael Gattozzi <mgattozzi@gmail.com>"]

[dependencies]
hyper = { git = "https://github.com/hyperium/hyper" }
hyper-tls = { git = "https://github.com/hyperium/hyper-tls" }
tokio-core = "0.1"
futures = "0.1"
```

Cool we've specified all the dependencies we'll actually need! Now let's setup the imports in our program. Open up your main.rs file and add the following lines at the top:

棒呆，我们已经指定了我们实际需要的所有依赖。 现在到我们的程序中设置导入。 打开你的main.rs文件，在顶部添加以下几行：

```rust
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future;

use hyper::{Url, Method, Error};
use hyper::client::{Client, Request};
use hyper::header::{Authorization, Accept, UserAgent, qitem};
use hyper::mime::Mime;
use hyper_tls::HttpsConnector;
```

Seems like a lot right? I thought so too, but Hyper is a low level HTTP library and we need this level of granularity to make sure our requests are setup right for the GitHub API. The good news is that it's not scary! Let's start building up our request so you can see where all these imports fit in.

看上去有很多是不是？ 我也这么认为，但是Hyper是一个很底层的HTTP库，我们需要这个级别的粒度来确保我们请求GitHub API的设置是正确的。 好消息是这不是可怕的！ 让我们开始建立我们的请求，以便您可以看到所有这些import导入的包如何使用。

First up let's begin crafting the request we'll need. In order to do this we'll need a Url to point the request at:

首先，让我们开始创建我们需要的request请求。 为了做到这一点，我们需要一个Url来指出requset请求地址：

```rust
fn main() {
    let url = Url::parse("https://api.github.com/user").unwrap();
```

Pretty self explanatory, pass it in a string and that becomes the `Url` struct. This functionality and all of it's methods was just like before in Hyper so you can easily extend the url dynamically or get other information from it. What, we care about here is that we have it pointing at the end point we want to use to get data [on ourselves](https://developer.github.com/v3/users/#get-the-authenticated-user).

就是代码字面意思，传入一个字符串，并转为Url结构体。 这个功能及其所有的方法就像在Hyper中一样，所以你可以很容易地动态扩展url或者从中获取其他信息。 我们关心的是，我们使用它指向我们想用来[我们自己](https://developer.github.com/v3/users/#get-the-authenticated-user)获取数据的请求地址。

Sweet! Now let's use that to make a [Request](https://hyper.rs/hyper/master/hyper/client/struct.Request.html) struct. This is what we'll use to set the headers to what we want for the API

很好！ 现在让我们使用它来创建一个[Request](https://hyper.rs/hyper/master/hyper/client/struct.Request.html)结构体。 我们将使用它来为API设置头部信息

```rust
    let mut req = Request::new(Method::Get, url);
```

It takes a `Method`, an enum representing all the different types of requests you can make like GET, POST, PUT, DELETE, PATCH, etc., and a `Url`. You have access to the handle and headers from this struct via function calls so that you can change aspects of it that you want to change. Alright let's get our [Mime](https://hyper.rs/mime.rs/mime/struct.Mime.html) value and authorization token setup for the headers

它接受一个`类型Method`，就是一个枚举代表你可以做的所有不同类型的请求，如GET，POST，PUT，DELETE，PATCH等，和一个`Url`。 你可以通过函数调用来访问这个结构中的句柄和头文件，这样你就可以修改它。 很好，让我们来为我们的头文件设置我们的[Mime](https://hyper.rs/mime.rs/mime/struct.Mime.html)值和授权token

```rust
    let mime: Mime = "application/vnd.github.v3+json".parse().unwrap();
        let token = String::from("token {Your_Token_Here}");
```

Why is this media type (`Mime`) needed? Well if you look at the [GitHub docs](https://developer.github.com/v3/media/) you can set what you want to receive back from the API. Usually we would want JSON, so we ask for that but we also set which version of the API to use with the `vnd.github.v3+` part. We're telling GitHub to use version 3 of the API because we don't want anything to break if all of a sudden they switch to version 4 for some reason.

为什么需要这种媒体类型（`Mime`）？ 如果你看看[GitHub文档](https://developer.github.com/v3/media/)，你可以设置你想从API接收的内容。 通常我们想要JSON，所以我们要求，但是我们也设置了哪个版本的API与 `vnd.github.v3+` 部分一起使用。 我们告诉GitHub使用API的第3版，因为如果出于某种原因切换到版本4，我们不希望任何内容被破坏。

We also need our token to be in the header. From trial and error when I first used Hyper in the library I realized that GitHub is expecting input of the form `token {Your_Token_Here}` for their `Authorization` header. It's a bit weird when I first tried to figure it out. Originally I thought I was supposed to use Hyper's `Bearer` struct since it had a token value inside of it but that was not the case apparently.

我们还需要在header头部添加token。 从我第一次在库中使用Hyper时的调试和报错中我意识到GitHub需要输入的`Authorization` 头信息是 `token {Your_Token_Here} `的格式。 当我第一次尝试弄明白的时候，这有点奇怪。 最初我以为我应该使用Hyper的 `Bearer` 结构，因为它里面有一个令牌值，但显然不是这样。

Let's change the headers of our Request now:

下面来修改请求头：

```rust
    req.headers_mut().set(UserAgent(String::from("ghub-example")));
    req.headers_mut().set(Accept(vec![qitem(mime)]));
    req.headers_mut().set(Authorization(token));
```

I'm doing this with the `headers_mut().set()` way due to some borrowing errors I ran into and moved values. Meaning I couldn't do:

我在用 `headers_mut().set()` 方法修改，因为我遇到了一些借用错误，由于转移了值。 意味着不允许我这么做：

```rust
let mut headers = req.headers_mut();
headers.set()
```

And then using `req` later, as `req` didn't exist anymore. Not sure if this was a rust or a Hyper issue but this works just fine. If you figure out a more ergonomic way to do it let me know!

后面使用`req`，而`req`已经不存在了。 不确定这是rust还是Hyper问题，但目前的方式是可以用的。 如果你有更好的方式，让我知道！

First up we need a [UserAgent](https://hyper.rs/hyper/master/hyper/header/struct.UserAgent.html) in our headers. Why? According to the [docs](https://developer.github.com/v3/#user-agent-required) GitHub will reject any request without it! You'll get a `403` when you try to make the request.

首先，我们需要在头信息中使用[UserAgent](https://hyper.rs/hyper/master/hyper/header/struct.UserAgent.html)。 为什么？ 根据[文档](https://developer.github.com/v3/#user-agent-required)GitHub会拒绝没有它的任何请求！ 当你尝试提出请求时，你会得到一个`403`。

Next up we are going to change our [Accept](https://hyper.rs/hyper/master/hyper/header/struct.Accept.html) header to utilize that Media type we had made earlier. We pass it to [qitem](https://hyper.rs/hyper/master/hyper/header/fn.qitem.html) which wraps it in a [QualityItem](https://hyper.rs/hyper/master/hyper/header/struct.QualityItem.html) type that `Accept` is expecting and then we put it in a `Vec` since `Accept` might hold multiple `QualityItem` values in the header of a request. We don't have multiple values here but it does need to be in a `Vec`.

接下来，我们要改变我们的[Accept](https://hyper.rs/hyper/master/hyper/header/struct.Accept.html)头信息，以利用我们之前做过的媒体类型。 我们把它传递给[qitem](https://hyper.rs/hyper/master/hyper/header/fn.qitem.html)，它把它包装在 `Accept` 所期望的 [QualityItem](https://hyper.rs/hyper/master/hyper/header/struct.QualityItem.html) 类型中，然后我们把它放在一个 `Vec` 中，因为 `Accept` 可以在请求的头部保存多个 `QualityItem` 值。 我们在这里没有多个值，但它需要在`Vec`中。

Lastly we set our [Authorization](https://hyper.rs/hyper/master/hyper/header/struct.Authorization.html) with our token by just passing it in to an `Authorization` struct. Boom we've setup all of our headers and crafted the request we need. Now let's start dealing with Futures.

最后，我们通过将它传递给授权 `Authorization` 结构体来设置我们的 [Authorization](https://hyper.rs/hyper/master/hyper/header/struct.Authorization.html)。 我们已经设置了我们所有的头信息，并制定了我们所需要的request。 现在我们开始处理 Futures。

```rust
    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();
```

First up we need to setup an event loop ([Core](https://docs.rs/tokio-core/0.1.4/tokio_core/reactor/struct.Core.html)) that will handle processing our Future when we need it. We'll also need a [Handle](https://docs.rs/tokio-core/0.1.4/tokio_core/reactor/struct.Handle.html) to that event loop so that our [Client](https://hyper.rs/hyper/master/hyper/client/struct.Client.html) and `HttpsConnector` know which event loop to be processed on.

首先，我们需要设置一个事件循环（[Core](https://docs.rs/tokio-core/0.1.4/tokio_core/reactor/struct.Core.html)），在我们需要的时候处理我们的Future。 我们还需要一个该事件循环的[Handle](https://docs.rs/tokio-core/0.1.4/tokio_core/reactor/struct.Handle.html)，以便我们的客户端[Client](https://hyper.rs/hyper/master/hyper/client/struct.Client.html)和 `HttpsConnector` 知道要处理哪个事件循环。

Alright let's set the Client up so we can make connections:

很好，现在来创建Client，以便建立连接：

```rust
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle))
        .build(&handle);
```

Since we're not using the default version of the client which only does HTTP we call the `configure()` function so that we can change the connector. In this case we're using `HttpsConnector` from the hyper-tls library, but presumably anything that implements the `Connect` trait should work. This might allow for requests by other protocols if I'm not mistaken. You might be wondering what that number four is for, well I had to look at the source code originally since there were no online docs for it yet. Here's what the relevant comment said, "Takes number of DNS worker threads." Four is what had been in an older example in the Hyper repo so I just went with that. You can change that to the number of your liking. We then tell it to build itself and we now have a `Client` with HTTPS support! We're almost done. Let's actually make our `Future`:

由于我们没有使用只处理HTTP的客户端默认版本，我们调用了 `configure()` 函数以便我们可以更改连接器。 在这个例子，我们使用hyper-tls库中的 `HttpsConnector`，推测是任何实现 `Connect` trait特性的都应该工作。 如果我没有弄错，这可能允许其他协议的请求。 你可能想知道数字 4 是什么意思，由于还没有在线文档我不得不查看源代码。 以下是相关的评论：“指定DNS工作线程数量”。 4 是在Hyper 库中一个更老的例子里的就有的，所以我也就直接用了。 你可以改成任意数量。 然后我们告诉它建立自己，我们现在有一个HTTPS支持的`Client客户端`！ 我们差不多完成了。 让我们来实现我们的 `Future`：

```rust
    let work = client.request(req)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            res.body().fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, Error>(v)
            }).and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                future::ok::<_, Error>(s)
            })
        });
```

The thing with futures is that it's always expecting some kind of future to pass on to the next function call chained to it and eventually it will pass on a value where it's completed when you run the future. So you can have futures in futures. If you look at the above code it's exactly what we did. First we tell our client to make a request and pass it our `Request` struct from earlier. This gives us a [FutureResponse](https://hyper.rs/hyper/master/hyper/client/struct.FutureResponse.html) which resolves to a [Response](https://hyper.rs/hyper/master/hyper/client/struct.Response.html) if it works out. When we call `and_then()` we're saying once you get the response back do this. In this case we're saying print out the status (did we get a 200, 403, 404 or something else?) and the headers from the response. We then call `res.body()` which creates another `Future` called a [Body](https://hyper.rs/hyper/master/hyper/struct.Body.html), which is a stream of [Chunks](https://hyper.rs/hyper/master/hyper/struct.Chunk.html), where a `Chunk` is basically a vector of bytes (`Vec<u8>`). If you look at the first part after `body()` we're getting each `Chunk` and folding the values into a single vector and putting it into a future `ok` so we can chain another computation. After that we want it to take that vector and turn it into a `String` and return that value in a `Future`! When run it'll return either an error or the JSON String from the call.

有Future的对象是，它总是期待某种future传递给链接到它的下一个函数调用，并最终将传递一个值，当你运行future的时候它会完成。所以你可以在future中使用future。如上面的代码，这正是我们所做的。首先我们告诉我们的client调用request，参数是上面创建的`Request`结构体`req`。这给了我们一个[FutureResponse](https://hyper.rs/hyper/master/hyper/client/struct.FutureResponse.html)，如果运行结束，它将解析为Response。当调用`and_then()`时，指定一旦获到response响应来做这个。在这个例子，我们打印状态码（有没有打印200,403,404或其他的状态码？）和response的头信息。然后我们调用`res.body()`，创建另一个调用[Body](https://hyper.rs/hyper/master/hyper/struct.Body.html)的Future，它是一个[Chunks](https://hyper.rs/hyper/master/hyper/struct.Chunk.html)的流，`Chunk` 是一个字节数组（`Vec<u8>`）。如果你看一下 `body()` 之后的第一部分，我们将得到每个 `Chunk`，并将这些值合成一个单独的数组，并将其放入future `ok`，以便我们可以链接另一个处理。之后，我们希望它把这个数组变成一个字符串，并在Future结构体返回这个值！运行时，会返回一个错误或来自调用的JSON字符串。

All right, let's run the future to completion and print out the result.

现在让我们运行Future来完成并打印出结果。

```rust
        let user = event_loop.run(work).unwrap();
        println!("We've made it outside the request! \
                  We got back the following from our \
                  request:\n");
        println!("{}", user);
    }
```

We pass in the future to the event loop and get back the value from it, in this case a `String` and then print it out. Sweet. Let's see it in action then!

我们把future传递给事件循环，并获得返回值，在这个例子是一个`String`，然后将其打印出来。 很好，那我们来看看吧！

Save the file then do:

保存文件并运行：

```shell
cargo run
```

You'll get output similar to this:

会得到如下输出

```text
Response: 200 OK
Headers: 
Server: GitHub.com
Date: Thu, 09 Mar 2017 18:59:58 GMT
Content-Type: application/json; charset=utf-8
Content-Length: 1450
Status: 200 OK
X-RateLimit-Limit: 5000
X-RateLimit-Remaining: 4999
X-RateLimit-Reset: 1489089598
Cache-Control: private, max-age=60, s-maxage=60
Vary: Accept, Authorization, Cookie, X-GitHub-OTP
Vary: Accept-Encoding
ETag: "a6fbebdd7e3ea78f873e2531b6af2562"
Last-Modified: Wed, 15 Feb 2017 16:43:54 GMT
X-OAuth-Scopes: admin:gpg_key, admin:org, admin:org_hook, admin:public_key, admin:repo_hook, delete_repo, gist, notifications, repo, user
X-Accepted-OAuth-Scopes: 
X-GitHub-Media-Type: github.v3; format=json
Access-Control-Expose-Headers: ETag, Link, X-GitHub-OTP, X-RateLimit-Limit, X-RateLimit-Remaining, X-RateLimit-Reset, X-OAuth-Scopes, X-Accepted-OAuth-Scopes, X-Poll-Interval
Access-Control-Allow-Origin: *
Content-Security-Policy: default-src 'none'
Strict-Transport-Security: max-age=31536000; includeSubdomains; preload
X-Content-Type-Options: nosniff
X-Frame-Options: deny
X-XSS-Protection: 1; mode=block
X-Served-By: 02ea60dfed58b2a09106fafd6ca0c108
X-GitHub-Request-Id: 8572:356D:62252BA:747B25E:58C1A62E

We've made it outside the request! We got back the following from our request:

我们完成了request请求！ 然后得从request得到以下结果：

{"login":"thearesia","id":25337282,"avatar_url":"https://avatars1.githubusercontent.com/u/25337282?v=3","gravatar_id":"","url":"https://api.github.com/users/thearesia","html_url":"https://github.com/thearesia","followers_url":"https://api.github.com/users/thearesia/followers","following_url":"https://api.github.com/users/thearesia/following{/other_user}","gists_url":"https://api.github.com/users/thearesia/gists{/gist_id}","starred_url":"https://api.github.com/users/thearesia/starred{/owner}{/repo}","subscriptions_url":"https://api.github.com/users/thearesia/subscriptions","organizations_url":"https://api.github.com/users/thearesia/orgs","repos_url":"https://api.github.com/users/thearesia/repos","events_url":"https://api.github.com/users/thearesia/events{/privacy}","received_events_url":"https://api.github.com/users/thearesia/received_events","type":"User","site_admin":false,"name":"Thearesia \"Sword Saint\" van Astrea","company":null,"blog":"https://github.com/mgattozzi/thearesia","location":"Kingdom of Lugnica","email":null,"hireable":null,"bio":"I'm a Github bot maintained by @mgattozzi","public_repos":0,"public_gists":0,"followers":1,"following":0,"created_at":"2017-01-25T03:25:48Z","updated_at":"2017-02-15T16:43:54Z","private_gists":0,"total_private_repos":0,"owned_private_repos":0,"disk_usage":0,"collaborators":0,"two_factor_authentication":true,"plan":{"name":"free","space":976562499,"collaborators":0,"private_repos":0}}
```

Awesome it all worked out perfectly! Here's what the code looks like all together:

棒呆，这一切都完美了！ 完整示例如下：

```rust
extern crate hyper;
extern crate hyper_tls;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::{Future, Stream};
use futures::future;

use hyper::{Url, Method, Error};
use hyper::client::{Client, Request};
use hyper::header::{Authorization, Accept, UserAgent, qitem};
use hyper::mime::Mime;
use hyper_tls::HttpsConnector;

fn main() {
    let url = Url::parse("https://api.github.com/user").unwrap();
    let mut req = Request::new(Method::Get, url);
    let mime: Mime = "application/vnd.github.v3+json".parse().unwrap();
    let token = String::from("token {Your_Token_Here}");
    req.headers_mut().set(UserAgent(String::from("github-rs")));
    req.headers_mut().set(Accept(vec![qitem(mime)]));
    req.headers_mut().set(Authorization(token));

    let mut event_loop = Core::new().unwrap();
    let handle = event_loop.handle();
    let client = Client::configure()
        .connector(HttpsConnector::new(4,&handle))
        .build(&handle);
    let work = client.request(req)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            res.body().fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, Error>(v)
            }).and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                future::ok::<_, Error>(s)
            })
        });
    let user = event_loop.run(work).unwrap();
    println!("We've made it outside the request! \
              We got back the following from our \
              request:\n");
    println!("{}", user);
}
```

## Conclusion 总结

Future's is changing the game in the Rust world and Hyper is stepping up to the plate. Once I wrapped my head around it worked it became really easy to work with. It really helps if you understand how futures work and if you plan on upgrading to this I'd recommend having a solid understanding how tokio and futures work together here with Hyper. Hopefully you've gotten a better understanding how to use the library and come up with some even more cool or complex things beyond this. I encourage you to try it out and start prepping your projects for the eventual upgrade. I've also posted the code on [GitHub](https://github.com/mgattozzi/ghub) for you if you want to just clone the repo. It won't work at all till you add your token though, so don't try to run it as is!

Future正在改变rust世界的游戏而Hyper正在变得更加优秀。 当我考虑使用，它就变得非常容易。 如果你了解Future的工作原理，并且如果你打算升级到这个方案，那么这真的有所帮助。我建议你扎实了解tokio和future在这里和Hyper如何一起配合。 希望你能更好地理解如何使用这个库，并且想出比这个更酷或更复杂的东西。 我鼓励你尝试一下，并开始准备你的项目，以便最终升级。 如果你想克隆仓库，我也已经在[GitHub](https://github.com/mgattozzi/ghub)上为你发布了代码。 你需要添加自己的token来让它运行！

