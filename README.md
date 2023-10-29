# Adversarial Armor

[![IMAGE ALT TEXT HERE](https://jphacks.com/wp-content/uploads/2023/07/JPHACKS2023_ogp.png)](https://www.youtube.com/watch?v=yYRQEdfGjEg)

## 製品概要
Adversarial ArmorはReactを用いたWebアプリケーションである．
このアプリは，ユーザーが画面に表示された8枚のノイズがのった画像の中からAIが誤認識するであろう画像を選択し，AIの認識能力を試すシステムである．

### 背景(製品開発のきっかけ、課題等）
現代社会において驚異的な広がりを見せているAI．近年では，その手は監視カメラを通した人々の監視にも迫っている．完全なる監視は人権侵害であり，我々のプライベートを守る必要があるため人間は，AIに対し防衛意識を向上させなければならない．そのためここでは，人間が認識できるかつ画像にどのような画像を加えればAIに誤認識させられるのかを学び，システムを通して認識阻害をAIの脅威から身を守る方法の1つとして備えたい．<br>
これを目標とし，今の技術には人間とbotを見分ける技術が存在しているが，AIまでも騙すというシステムがまだ存在していないところに目をつけ，このシステムを開発するに至った．
### 製品説明（具体的な製品の説明）
このシステムでは，画面にお題の画像が8つ表示される．この8つの画像の中からAIが誤認識しそうな画像を選択し，"VERIFY"ボタンを押して答え合わせに入る．このとき，AIが誤認識する画像の数は1から8つのランダムである．<br>
画像を選択すると，選択した画像のウインドウの左上にチェックがつく．答え合わせ後，誤認識した画像のウインドウの下に〇がつき，解答が示される．"BACK"ボタンを押すと元の画面に戻る．
### 特長
#### 1. 特長1　学習的側面
このシステムを通してユーザーは，AIの画像認識技術について理解を深めることができます。AIが画像認識においてどのように機能し、どの要素が誤認識につながるのかを実際の体験を通じて学ぶことができる。また、AIの限界を知り、自身や他の人々のセキュリティを向上させるためのスキルを習得できる。

#### 2. 特長2　挑戦とエンターテインメント
このシステムでは、エンターテインメント要素を組み込むことで学習を楽しさとして提供する。ユーザーはAIに対する挑戦を楽しむ一方、AIを騙す方法を見つけ出す過程で楽しさを体験することができる。

#### 3. 特長3　拡張性
このシステムには，研究者やエンジニアが土工事のAIモデルを組み込むことも可能になっている．これにより，より高度なテストや研究が行いやすくなり，AIの発展にも貢献することができる．

### 解決出来ること
人間が判別できる画像の範囲と現在のAIが判別できる画像の範囲の差を様々なノイズを用いて集積したデータを提供することで人間側のAIに対する防衛意識を向上させることができる．<br>
また，人間のみに判別できる画像のノイズ範囲を知ることで，認識阻害方法の1つとしてノイズを用いることができるようになる．
### 今後の展望
今回では，特定のテーマに絞って画像認識のノイズによる阻害を行ったが，これを実際に人間の顔にノイズを用いた際の認識についてAIの認識範囲について調べ，AIによる人間の管理から現実的に自己を防衛できるようにしていく．
<br>また，今回はWebアプリにて開発を行ったが，マイクロサービス化をすることでAIからの攻撃を防ぐ身近なシステムとしての実用化を目指す．
### 注力したこと（こだわり等）
* 
* 

## 開発技術
### 活用した技術
#### API・データ
* 
* 

#### フレームワーク・ライブラリ・モジュール
* React
* React-Router-DOM
  
#### デバイス
* 
* 

### 独自技術
#### ハッカソンで開発した独自機能・技術
* 独自で開発したものの内容をこちらに記載してください
* 特に力を入れた部分をファイルリンク、またはcommit_idを記載してください。

#### 製品に取り入れた研究内容（データ・ソフトウェアなど）（※アカデミック部門の場合のみ提出必須）
* 
* 
