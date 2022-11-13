import random

from bokeh.driving import count
from bokeh.io import curdoc
from bokeh.models.widgets import Div

#function for streaming
def get_price(t):
    value = 10.7 + t / 100 - (t / 100 ) ** 0.5 - random.uniform(0.001,0.05)
    return round(value*100)/100

stock_name = 'DAU'
price = '10.7'
percentage = "20"
#template for the text and numbers that appear
template=("""
      <div class='content'>
       <div class='name'> {stock_name} </div>
        <span class='number'>{price}<small>{price_unit}</small> </span>
        <span class='percentage' style='color: {colour};'> {percentage}<small>%</small> </span>
      </div>
      """)
# initial text
text = template.format(stock_name = stock_name,
                   price=price,
                   price_unit='k',
                   percentage=percentage,
                   colour='#97D389')
div = Div(text=text, height=300)

@count()
def update(t):
    # periodic callback to update price and any other detail.
    price = get_price(t)
    previous_price = get_price(t-1)
    percentage = (price - previous_price)/previous_price
    percentage = round(percentage*100)/100*100 + 20
    if(price > previous_price):
        color = '#97D389'
    elif(price == previous_price):
        color = '#FFFFFF'
    else:
        color = '#E31714'
    div.text = template.format(stock_name = stock_name,
                   price=price,
                   price_unit='k',
                   percentage=percentage,
                   colour=color)
    
curdoc().add_periodic_callback(update, 2000)
curdoc().add_root(div)