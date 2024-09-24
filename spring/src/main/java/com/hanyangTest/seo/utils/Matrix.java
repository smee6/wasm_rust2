package com.hanyangTest.seo.utils;

public class Matrix {

	public static double[][] multiply(double[][] a, double[][] b) {
		int n = a.length;
		double[][] c = new double[n][n];

		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				for (int k = 0; k < n; k++) {
					c[i][j] += a[i][k] * b[k][j];
				}
			}
		}

		return c;
	}

	public static void main(String[] args) {
		int n = 1000;
		double[][] a = new double[n][n];
		double[][] b = new double[n][n];

		// Initialize matrices with some values
		for (int i = 0; i < n; i++) {
			for (int j = 0; j < n; j++) {
				a[i][j] = 1.0;
				b[i][j] = 2.0;
			}
		}

		long startTime = System.currentTimeMillis();
		double[][] c = multiply(a, b);
		long endTime = System.currentTimeMillis();

		System.out.println("Matrix multiplication completed in " + (endTime - startTime) + " milliseconds." + "Result[0][0]: " + c[0][0]);
		// Optionally print the result matrix
		// for (int i = 0; i < n; i++) {
		//     for (int j = 0; j < n; j++) {
		//         System.out.print(c[i][j] + " ");
		//     }
		//     System.out.println();
		// }
	}
}
