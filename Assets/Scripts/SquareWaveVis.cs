using UnityEngine;
using System.Collections;

public class SquareWaveVis : MonoBehaviour
{
    public int speedFactor;
    private bool leftward;
	private int C;

	// Use this for initialization
	void Start () {
        if (gameObject.transform.parent.GetComponent<Wave>() != null)
        {
            leftward = gameObject.transform.parent.GetComponent<Wave>().getLeftward();
        }
        else if (gameObject.transform.parent.GetComponent<WaveSpawn>() != null)
        {
            leftward = gameObject.transform.parent.GetComponent<WaveSpawn>().leftward;
        }
        transform.Translate(leftward ? 0.4f : -0.4f, 0, -0.5f);
	}

	// Update is called once per frame
	void Update () {
		if (C > 80) {
            transform.Translate (leftward? 1 : -1, 0, 0);
			C = 1;
		}
		if (C <= 20 && C > 0) {
            transform.Translate (0, 0, 0.05f * speedFactor);
		}
		if (C <= 40 && C > 20) {
            transform.Translate (leftward? -0.025f * speedFactor : 0.025f * speedFactor, 0, 0);
		}
		if (C <= 60 && C > 40) {
            transform.Translate (0, 0, -0.05f * speedFactor);
		}
		if (C > 60) {
            transform.Translate (leftward? -0.025f * speedFactor: 0.025f * speedFactor, 0, 0);
		}
		C += speedFactor ;
	}
}
