# bioactivity-prediction-app

**This app was made with the help of @dataprofessor 's  bioacitvity prediction ML tutorial**

# Reproducing this web app
To recreate this web app on your own computer, do the following.

### Create conda environment
Firstly, we will create a conda environment called *bioactivity*
```
conda create -n bioactivity python=3.7.9
```
Secondly, we will login to the *bioactivity* environement
```
conda activate bioactivity
```
### Install prerequisite libraries

Download requirements.txt file

```
wget https://raw.githubusercontent.com/dataprofessor/bioactivity-prediction-app/main/requirements.txt

```

Pip install libraries
```
pip install -r requirements.txt
```

###  Download and unzip contents from GitHub repo

Download and unzip contents from https://github.com/dataprofessor/bioactivity-prediction-app/archive/main.zip

### Generating the PKL file

The machine learning model used in this web app will firstly have to be generated by successfully running the included Jupyter notebook [bioactivity_prediction_app.ipynb](https://github.com/dataprofessor/bioactivity-prediction-app/blob/main/bioactivity_prediction_app.ipynb). Upon successfully running all code cells, a pickled model called acetylcholinesterase_model.pkl will be generated.

###  Launch the app

```
streamlit run app.py
```
