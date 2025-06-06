###### Q: 你的投票系统即使在开发完成后我还是能找到很多缺点的。

A: VAVS 并不是要做到完美，而是要比你们现在能够找到的其他投票方式相对更好，这就够了。你可以自己跟AI聊聊，你们现在可以选择的其他投票方式都有哪些难以弥补的缺点，而这些缺点在VAVS中都被科技手段解决了

###### Q: 我不相信洋葱网络可以很好地保护隐私，让人们真正能匿名投票。

A: 没关系。之后开发的新功能中会包括打印功能，也就是说选民不需要将选票通过洋葱网络传输给选举组织者。可以将自己的选举签名打印成二维码，然后使用传统的方式，邮寄或者亲自投到投票箱里。而我们只要在制度上做好监督，让任何想要通过任何手段分析选票上的指纹或其他信息的人，无法对选民去匿名化就够了。

###### Q: 如果想要选举舞弊的人推动立法禁止使用这种投票方式怎么办？而且本来这种投票方式由于可以让选民证明自己投给了谁，是可以很好地用于贿赂选举的。

A: 这个问题问得很好。

* 首先，我要回答后一个问题。贿赂选举虽然因为这种投票方式变得更容易了，但我们同时可以开放钓鱼执法，让政府或者非政府组织自己去扮演想要贿赂选举的人，去贿赂选民，同时记录证据。这样一来，由于坏人之间不会相互信任，所以即使有选民想要出卖自己的选票，他也无法信任购买选票的人。

* 其次，即使法律上真的禁止了这种投票方式，但这个系统仍然可以被用于做民调。而如果多次的民调结果都显示是A获胜，最后的传统投票结果却是B获胜，那么这就足以说明非常多的问题了。

* 而且，它不仅仅是做民调的作用，它还可以在选举之前，让人们先在网络上更低成本、更高效地组织起来凝聚共识，从而避免“浪费选票”的担忧，这对于中间温和派的小党成为第三大党是非常有利的

* 低成本地、高速地凝聚共识还可以让人们更轻松地抗议，人们可以通过这套匿名机制更安全更高效得多地决定是否要罢工、罢课、上街抗议，这些都会显著地改变博弈中的优势差距

###### Q: 如果我的电脑或手机被黑客入侵了怎么办？如果苹果或微软的内部人员被买通，或者黑客使用了昂贵的 0day 漏洞攻击我怎么办？

A: 这是一个非常现实的问题。但 VAVS 的设计哲学考虑到了这一点。请注意以下关键区别：

* **攻击者最多只能“看”，无法“改”而不被发现：** 即使是最坏的情况，例如攻击者通过侵入您的设备知道了您的私钥，从而知晓了您的投票选择。但是，他们**无法篡改已经投出的、经过您签名的选票而不留下痕迹**。他们也**无法阻止您发现异常**（比如您发现自己的密钥镜像被用于投票，或者您自己投票后发现有重复投票链接）。
* **篡改选票并影响结果的难度极高：** 攻击者要想在不被发现的情况下大规模篡改选票来影响选举结果，需要同时攻破大量选民的设备**并**阻止他们发现异常和报告，**或者**攻破并篡改由选举组织者发布的、经过密码学保护且公开可验证的计票结果。这比仅仅知道个别人的投票选择要困难很多个数量级。
* **安全性的焦点在于“结果可信”：** VAVS 的核心优势在于保障**选举结果的完整性和可验证性**。相比于保护个人设备隐私（这本身就是一个极其困难且涉及多方面的挑战，想想保护聊天软件的隐私有多难），确保投票和计票过程的密码学安全、防止结果被篡改，是一个更明确、更可实现的目标。VAVS 正是专注于此。

###### Q: 我不喜欢你这个人，在我眼里，我可以找到你的非常多的问题。

A: 我是一个什么样的东西并不重要。重要的是，我能带给你、带给这个世界什么。而且我所做的工作都是公开透明的，任何有能力的人都可以通过公开信息进行分析。

我曾经想到过一个比喻：有这么一个弱女子，在一个废弃的厂房里被一群强奸犯追逐，然后你躲进了一个死胡同中的房间，那帮人已经在房间门外砸门想试图闯进来了。这时，突然冒出一个背生双翼、长着犄角的魔鬼，当着你的面，将一挺重机枪在十秒内组装好摆在你面前，最后对你露出一个邪恶的冷笑，然后消失不见，那么你会因为这个留下重机枪的是什么东西，而不去使用这挺重机枪吗？
