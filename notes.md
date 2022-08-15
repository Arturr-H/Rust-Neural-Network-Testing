# Notes for learning AI


## Weights
**"Weights"** är bindelserna mellan två neuroner. Men weight kan också menas hur viktig en neuron är för den nästa, eller hur mycket den kommer influenca den andra neuronen. T.ex: n<sub>1</sub> &nbsp;=== weight ===>&nbsp; n<sub>2</sub>. N<sub>1</sub> kommer påverka n<sub>2</sub> beroende på hur stark weighten är (n1 * weight).

Om två st neuroner pekar på en neuron, såklart med två **weights** var, så blir output neuronen det här: output = (n<sub>1</sub> * w<sub>1</sub>) + (n<sub>2</sub> * w<sub>2</sub>)


## Biases
todo

## Activation functions
Efter att vi har tagit summan av en neuron, genom att göra det här: (n<sub>1</sub> * w<sub>1</sub>) + (n<sub>2</sub> * w<sub>2</sub>) => Output, så får vi något float-nummer, som kan ligga mellan -100... - 100..., typ oändligt stor. Man tar det värdet och kör det genom en s.k activation function, det finns flera olika men sigmoid är mest använt. Den liskom reagerar till värdet som den får som input. När värdet passerar en viss gräns så ökar eller minskar return värdet av funktionen. Här är en bra bild för demonstration: <a href="https://i.stack.imgur.com/ddyfr.png">Bild</a> 