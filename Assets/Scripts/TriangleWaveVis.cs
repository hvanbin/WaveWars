using UnityEngine;
using System.Collections;

public class TriangleWaveVis : MonoBehaviour
{
    public int speedFactor;
    private int C;
    private bool leftward;
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
        transform.Translate (leftward? 0.4f : -0.4f, 0, -0.5f);
	}

	// Update is called once per frame
	void Update ()
    {
		if (C > 80) {
			transform.Translate (leftward? 1: -1, 0, 0);
			C = 1;
		}
		if (C <= 40 && C > 0) {
			transform.Translate (leftward? -0.0125f *speedFactor: 0.0125f * speedFactor, 0, 0.02f * speedFactor);
		}
		if (C > 40) {
			transform.Translate (leftward? -0.0125f * speedFactor: 0.0125f*speedFactor, 0, -0.02f*speedFactor);
		}
		C += speedFactor;
	}
}
