# Notes for learning AI


## Weights
**"Weights"** är bindelserna mellan två neuroner. Men weight kan också menas hur viktig en neuron är för den nästa, eller hur mycket den kommer influenca den andra neuronen. T.ex: n<sub>1</sub> &nbsp;=== weight ===>&nbsp; n<sub>2</sub>. N<sub>1</sub> kommer påverka n<sub>2</sub> beroende på hur stark weighten är (n1 * weight).

Om två st neuroner pekar på en neuron, såklart med två **weights** var, så blir output neuronen det här: output = (n<sub>1</sub> * w<sub>1</sub>) + (n<sub>2</sub> * w<sub>2</sub>)


## Biases
Varje neuron har varsin bias. Liksom **weights**, så är bias det andra *"lärbara"* värdet. Biases fungerar som m värdet i räta linjens ekvation (y = kx + m). Alltså hur mycket värdet shiftar ifrån origo. Biaser gör så att ens modell blir mer flexibel. Men hela meningen med biases är att kunna ändra när en neuron ska aktiveras. Alla neuroner får sitt värde genom activation functions, som jag förklarar längre ner i sidan. Det ger ett värde mellan 0 och 1. Om man använder ex ReLU, och lägger ett värde av -0.2 i aktivering funktionen, så kommer det att ge tillbaka ett värde av 0.0.0, alltså att neuronen inte sätts på. med en bias så kan man "shifta" värdet, om man tycker att den är viktig och ändå ska aktiveras. ReLU(-0.2 + b) b = 0.6, out = 0.4.

## Activation functions
Efter att vi har tagit summan av en neuron, genom att göra det här: (n<sub>1</sub> * w<sub>1</sub>) + (n<sub>2</sub> * w<sub>2</sub>) => Output, så får vi något float-nummer, som kan ligga mellan -100... - 100..., typ oändligt stor. Man tar det värdet och kör det genom en s.k activation function, det finns flera olika men sigmoid är mest använt. Den liskom reagerar till värdet som den får som input, och "squishar" värdet till ett värde mellan 0 och 1 (float). När värdet passerar en viss gräns så ökar eller minskar return värdet av funktionen. Här är en bra bild för demonstration: <a href="https://i.stack.imgur.com/ddyfr.png">Bild</a> 

Så, en neurons värde kan beskrivas med en funktion:
fn_a( activation_function( w<sub>0</sub>a<sub>0</sub> + w<sub>1</sub>a<sub>1</sub> +.... + b )); b är biasen