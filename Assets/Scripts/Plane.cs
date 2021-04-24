using UnityEngine;
using System.Collections;
using UnityEngine.SceneManagement;

public class Plane : MonoBehaviour
{

    public int clockRate;
    public static int WIDTH = 5;
    public static int HEIGHT = 5;

    public static int AI = 1; // 0 is off, 1 is on
    public static int WINNER = 0; //set by the wave class to determine who wins at the end of the game.
    public int getClock()
    {
        return (int) updateClock;
    }
    private float updateClock;
    private GameObject buttonPanel2;

	private GameObject spawning;
	private string spawnName;
	private int xOffset;

	// Use this for initialization
	void Start ()
    {
        updateClock = 0.0f;
        buttonPanel2 = GameObject.Find("ButtonPanel2");
    }

    private void AIplay()
    {
        switch(AI)
        {
            case 0:
                return;
            case 1:
                GameObject spawner = buttonPanel2.transform.GetChild(Random.Range(0, buttonPanel2.transform.childCount)).gameObject;
                if(spawner.GetComponent<WaveSpawn>() != null)
                    spawner.GetComponent<WaveSpawn>().Spawn(true);
                break;
            case 2:
                /*for (int i = 0; i < buttonPanel2.transform.childCount; i++)
                {
                    GameObject spawner
                }*/
                break;
            case 3:
                break;
            default:
                return;
        }
    }

	void Update ()
	{	
		clockRate = Slider.clockRate;
        updateClock += Time.deltaTime;
        AIplay();
        if (updateClock >= clockRate)
        {
            if (WINNER != 0) SceneManager.LoadScene(SceneManager.GetActiveScene().buildIndex+1);

			if(WaveSpawn.LEFT_TYPE!=0){
				switch (WaveSpawn.LEFT_TYPE)
				{
				case 1:
					spawning = Resources.Load ("Waves/Sawtooth") as GameObject;
					spawnName = "Sawtooth";
					GameObject.Find ("Sound (1)").GetComponent<AudioSource> ().clip = Resources.Load ("Sounds/Sawtooth" + (int)((WaveSpawn.LEFT_ROW/2+3)*-1+6)) as AudioClip;
					break;
				case 2:
					spawning = Resources.Load("Waves/Square") as GameObject;
					spawnName = "Square";
					GameObject.Find ("Sound (1)").GetComponent<AudioSource> ().clip = Resources.Load ("Sounds/Square" + (int)((WaveSpawn.LEFT_ROW/2+3)*-1+6)) as AudioClip;
					break;
				default:
					spawning = Resources.Load ("Waves/Triangular") as GameObject;
					spawnName = "Triangular";
					GameObject.Find ("Sound (1)").GetComponent<AudioSource> ().clip = Resources.Load ("Sounds/Triangular" + (int)((WaveSpawn.LEFT_ROW /2+3)*-1+6)) as AudioClip;
					break;
				}
				GameObject wave = Instantiate(spawning, this.transform, false) as GameObject;
				GameObject.Find ("Sound (1)").GetComponent<AudioSource> ().Play ();
				wave.name = spawnName;
				wave.transform.position = new Vector3(4, 0, WaveSpawn.LEFT_ROW);
				wave.GetComponent<Wave>().setLeftward(true);
				wave.transform.GetChild (0).GetComponent<ParticleSystem> ().startColor = Resources.Load<Material> ("Materials/Gray").color;
				wave.transform.GetChild (1).GetComponent<ParticleSystem> ().startColor = Resources.Load<Material> ("Materials/Gray").color;
				WaveSpawn.LEFT_TYPE = 0;
				WaveSpawn.LEFT_ROW = 0;
			}if (WaveSpawn.RIGHT_TYPE != 0) {
				switch (WaveSpawn.RIGHT_TYPE) {
				case 1:
					spawning = Resources.Load ("Waves/Sawtooth") as GameObject;
					spawnName = "Sawtooth";
					GameObject.Find ("Sound (2)").GetComponent<AudioSource> ().clip = Resources.Load ("Sounds/Sawtooth" + (int)((WaveSpawn.LEFT_ROW/2+3)*-1+6)) as AudioClip;
					break;
				case 2:
					spawning = Resources.Load ("Waves/Square") as GameObject;
					spawnName = "Square";
					GameObject.Find ("Sound (2)").GetComponent<AudioSource> ().clip = Resources.Load ("Sounds/Square" + (int)((WaveSpawn.LEFT_ROW/2+3)*-1+6)) as AudioClip;
					break;
				default:
					spawning = Resources.Load ("Waves/Triangular") as GameObject;
					spawnName = "Triangular";
					GameObject.Find ("Sound (2)").GetComponent<AudioSource> ().clip = Resources.Load ("Sounds/Triangular" + (int)((WaveSpawn.LEFT_ROW /2+3)*-1+6)) as AudioClip;
					break;
				}
				GameObject wave = Instantiate (spawning, this.transform, false) as GameObject;
				GameObject.Find ("Sound (2)").GetComponent<AudioSource> ().Play ();
				wave.name = spawnName;
				wave.transform.position = new Vector3 (-4, 0, WaveSpawn.RIGHT_ROW);
				wave.GetComponent<Wave> ().setLeftward (false);
				wave.transform.GetChild (0).GetComponent<ParticleSystem> ().startColor = Resources.Load<Material> ("Materials/Brown").color;
				wave.transform.GetChild (1).GetComponent<ParticleSystem> ().startColor = Resources.Load<Material> ("Materials/Brown").color;
				WaveSpawn.RIGHT_TYPE = 0;
				WaveSpawn.RIGHT_ROW = 0;
			}
            for (int i = 0; i < transform.childCount; i++)
            {
                GameObject w = transform.GetChild(i).gameObject;
                w.GetComponent<Wave>().Move();
            }
            WaveSpawn.LEFT_READY = true;
            WaveSpawn.RIGHT_READY = true;
            updateClock = 0.0f;
        }
	}
}
