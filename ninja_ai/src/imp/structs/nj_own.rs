///All:一度実行されると全員が所有権を持つ。もう実行できない
///One:最初に実行した一人が所有権を持つ。もう実行できない
///None:誰も所有権を持たない。何度でも実行できる
///Unlimited:誰かが所有権を得ても、また別の人は実行でき、実行すれば所有権が得られる
///NotRunnable:実行できない。Wallがない
pub enum NjOwn{
    One,
    All,
    None,
    Unlimited,
}