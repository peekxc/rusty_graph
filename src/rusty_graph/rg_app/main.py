import random
import numpy as np 
from typing import *
from numpy.typing import ArrayLike
# import sys
# print(sys.path)
def rank_C2(i: int, j: int, n: int):
  i, j = (j, i) if j < i else (i, j)
  return(int(n*i - i*(i+1)/2 + j - i - 1))

def unrank_C2(x: int, n: int):
  i = int(n - 2 - np.floor(np.sqrt(-8*x + 4*n*(n-1)-7)/2.0 - 0.5))
  j = int(x + i + 1 - n*(n-1)/2 + (n-i)*((n-i)-1)/2)
  return(i,j) 

def edges_from_triangles(triangles: ArrayLike, nv: int):
  ER = np.array([[rank_C2(*t[[0,1]], n=nv), rank_C2(*t[[0,2]], n=nv), rank_C2(*t[[1,2]], n=nv)] for t in triangles])
  ER = np.unique(ER.flatten())
  E = np.array([unrank_C2(r, n=nv) for r in ER])
  return(E)

from bokeh.driving import count
from bokeh.io import curdoc
from bokeh.models import ColumnDataSource
from bokeh.models import *
from bokeh.models.widgets import Div
from bokeh.plotting import figure, from_networkx, show
from bokeh.palettes import Category20_20
from bokeh.transform import linear_cmap
from scipy.spatial import Delaunay

def render_complex(plot, X: ArrayLike, E: ArrayLike = [], T: ArrayLike = []):
  X_src = ColumnDataSource({ 'x': X[:,0], 'y': X[:,1] , 'lambda': X[:,0] })
  turbo_color = linear_cmap('lambda', 'Turbo256', low=-1.0, high=1.0) # DataSpec dict

  ## Triangles
  TX = [list(X[t,0]) for t in T] if len(T) > 0 else []
  TY = [list(X[t,1]) for t in T] if len(T) > 0 else []
  t_renderer = plot.patches(TX, TY, color="green", alpha=0.15, line_width=0)
  
  ## Edges 
  EX = [[X[u,0], X[v,0]] for u,v in E] if len(E) > 0 else []
  EY = [[X[u,1], X[v,1]] for u,v in E] if len(E) > 0 else []
  e_renderer = plot.multi_line(EX, EY, alpha=0.80, color="firebrick", line_width=1.5)

  ## Vertices; colored by lambda
  v_renderer = plot.circle('x', 'y', size=4, alpha=1.0, color=turbo_color, source=X_src) # TODO: learn about bokeh.transforms
  return v_renderer, e_renderer, t_renderer



X = np.random.uniform(size=(100, 2), low=-1.0, high=1.0)
dt = Delaunay(X)
T = dt.simplices
E = edges_from_triangles(T, X.shape[0])
p = figure(x_range=(-1.1, 1.1), y_range=(-1.1, 1.1), x_axis_location=None, y_axis_location=None, tools="hover", tooltips="index: @index")
p.grid.grid_line_color = None
vr,er,tr = render_complex(p, X, E, T)

def force_cb():
    cx = np.array(vr.data_source.data['x'])
    cy = np.array(vr.data_source.data['y'])
    cl = vr.data_source.data['lambda']
    cx += np.random.uniform(size=len(cx), low=-0.01, high=0.01)
    cy += np.random.uniform(size=len(cx), low=-0.01, high=0.01)
    vr.data_source.data = {
        'x' : cx,
        'y' : cy,
        'lambda' : cl
    }

    EX = [[cx[u], cx[v]] for u,v in E] if len(E) > 0 else []
    EY = [[cy[u], cy[v]] for u,v in E] if len(E) > 0 else []
    er.data_source.data = {
        'xs' : EX, 
        'ys' : EY
    }

curdoc().add_periodic_callback(force_cb, 250)
curdoc().add_root(p)

# configure attributes when initializing a model object
# glyph = Rect(x="x", y="y2", w=10, h=20, line_color=None)
# glyph.fill_alpha = 0.5
# glyph.fill_color = "navy"

# stock_name = 'DAU'
# price = '10.7'
# percentage = "20"
# #template for the text and numbers that appear
# template=("""
#       <div class='content'>
#        <div class='name'> {stock_name} </div>
#         <span class='number'>{price}<small>{price_unit}</small> </span>
#         <span class='percentage' style='color: {colour};'> {percentage}<small>%</small> </span>
#       </div>
#       """)
# # initial text
# text = template.format(stock_name = stock_name, price=price, price_unit='k', percentage=percentage, colour='#97D389')
# div = Div(text=text, height=300)

# @count()
# def update(t):
#     # periodic callback to update price and any other detail.
#     price = get_price(t)
#     previous_price = get_price(t-1)
#     percentage = (price - previous_price)/previous_price
#     percentage = round(percentage*100)/100*100 + 20
#     if(price > previous_price):
#         color = '#97D389'
#     elif(price == previous_price):
#         color = '#FFFFFF'
#     else:
#         color = '#E31714'
#     div.text = template.format(stock_name = stock_name,
#                    price=price,
#                    price_unit='k',
#                    percentage=percentage,
#                    colour=color)
    
# curdoc().add_periodic_callback(update, 2000)

# import networkx as nx

# G = nx.desargues_graph() # always 20 nodes

# p = figure(width=300, height=300, tools="pan,reset,save")
# cr = p.circle([1, 2.5, 3, 2], [2, 3, 1, 1.5], radius=0.3, alpha=0.5) # circle renderer