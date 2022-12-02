import pandas as pd
from sklearn.cluster._mean_shift import estimate_bandwidth


if __name__=='__main__':
    df = pd.read_csv('mnist_test.csv', sep=',')

    df = df.drop('label', axis = 1)
    X = list(df.itertuples(index=False, name=None))

    bandwidth = estimate_bandwidth(X, quantile=0.3, n_samples=300) * 5
    print("Bandwith calculated = ", bandwidth)
