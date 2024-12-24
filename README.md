# TeMP
CLIでテスト問題やクイズを作成したり、遊ぶために作られたソフトウェアです。


# License
LISENCE.txtを読んでください

# コマンド
Makeモードのコマンドについて解説します。<br>
<table>
  <tr>
    <td>コマンド</td>
    <td>構文</td>
    <td>解説</td>
    <td>使用例</td>
  </tr>
  <tr>
    <td>makq</td>
    <td>newq id "タイトル"</td>
    <td>クイズを作成します。</td>
    <td>makq test_quiz "ここはタイトル　テストクイズ"</td>
  </tr>
  <tr>
    <td>newq</td>
    <td>newq id "問題文" "答え" スコア</td>
    <td>クイズに問題を追加します。</td>
    <td>newq test_quiz "問題文です 1+1=?" "2" 10</td>
  </tr>
  <tr>
    <td>delq</td>
    <td>delq id 問題のインデックス(1から始まります)</td>
    <td>クイズに追加された問題を削除します。</td>
    <td>delq test_quiz 1</td>
  </tr>
  <tr>
    <td>import</td>
    <td>import id ファイル名</td>
    <td>作成されたクイズを読み込みます。</td>
    <td>import test_quiz test_quiz.json</td>
  </tr>
  <tr>
    <td>export</td>
    <td>export id</td>
    <td>クイズをJSONでエクスポートします。(タイトル+.json)</td>
    <td>export test_quiz</td>
  </tr>
</table>
