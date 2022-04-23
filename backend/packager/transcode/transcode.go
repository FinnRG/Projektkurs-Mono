package transcode

import (
	"log"
	"os"
	"os/exec"

	"msostream/packager/kafka"
	util "msostream/packager/minio"

	"github.com/minio/minio-go/v7"
)

func TranscodeAndUpload(id string) error {
	log.Printf("Started to work on: %s\n", id)

	// Create a new directory
	oserr := os.Mkdir(id, os.ModeDir)
	if oserr != nil {
		return oserr
	}

	// Get file from MinIO
	client, ctx := util.MinioClient()
	geterr := client.FGetObject(ctx, "raw", id, id+"/"+id, minio.GetObjectOptions{})
	if geterr != nil {
		return geterr
	}

	// Transcode
	err := transcode(id)
	if err != nil {
		return err
	}

	// Upload output files
	err = util.UploadFolder(id, client, ctx)
	if err != nil {
		return err
	}

	// Remove folder
	os.RemoveAll(id)

	err = kafka.EmitVideoProcessedEvent(id)
	if err != nil {
		return err
	}

	return nil
}

func createInputConfig(id string) error {
	log.Printf("Started creating input file: %s\n", id)
	f, err := os.Create(id + "/input.yaml")
	if err != nil {
		return err
	}
	defer f.Close()
	f.WriteString("inputs:")
	f.WriteString("\n  - name: \"" + id + "/" + id + "\"")
	f.WriteString("\n    media_type: video\n")
	log.Printf("Input file creation successful: %s\n", id)
	return nil
}

func transcode(id string) error {
	createInputConfig(id)
	log.Printf("Started transcoding: %s\n", id)
	cmd := exec.Command("/bin/shaka-streamer", "-i", "/"+id+"/input.yaml", "-p", "/conf/pipeline.yaml", "-o", "/"+id+"/output/")
	cmd.Env = os.Environ()
	out, err := cmd.Output()
	log.Print(out)
	return err
}
