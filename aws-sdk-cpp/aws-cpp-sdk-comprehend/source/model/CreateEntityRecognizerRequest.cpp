﻿/**
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#include <aws/comprehend/model/CreateEntityRecognizerRequest.h>
#include <aws/core/utils/json/JsonSerializer.h>

#include <utility>

using namespace Aws::Comprehend::Model;
using namespace Aws::Utils::Json;
using namespace Aws::Utils;

CreateEntityRecognizerRequest::CreateEntityRecognizerRequest() : 
    m_recognizerNameHasBeenSet(false),
    m_dataAccessRoleArnHasBeenSet(false),
    m_tagsHasBeenSet(false),
    m_inputDataConfigHasBeenSet(false),
    m_clientRequestToken(Aws::Utils::UUID::RandomUUID()),
    m_clientRequestTokenHasBeenSet(true),
    m_languageCode(LanguageCode::NOT_SET),
    m_languageCodeHasBeenSet(false),
    m_volumeKmsKeyIdHasBeenSet(false),
    m_vpcConfigHasBeenSet(false)
{
}

Aws::String CreateEntityRecognizerRequest::SerializePayload() const
{
  JsonValue payload;

  if(m_recognizerNameHasBeenSet)
  {
   payload.WithString("RecognizerName", m_recognizerName);

  }

  if(m_dataAccessRoleArnHasBeenSet)
  {
   payload.WithString("DataAccessRoleArn", m_dataAccessRoleArn);

  }

  if(m_tagsHasBeenSet)
  {
   Array<JsonValue> tagsJsonList(m_tags.size());
   for(unsigned tagsIndex = 0; tagsIndex < tagsJsonList.GetLength(); ++tagsIndex)
   {
     tagsJsonList[tagsIndex].AsObject(m_tags[tagsIndex].Jsonize());
   }
   payload.WithArray("Tags", std::move(tagsJsonList));

  }

  if(m_inputDataConfigHasBeenSet)
  {
   payload.WithObject("InputDataConfig", m_inputDataConfig.Jsonize());

  }

  if(m_clientRequestTokenHasBeenSet)
  {
   payload.WithString("ClientRequestToken", m_clientRequestToken);

  }

  if(m_languageCodeHasBeenSet)
  {
   payload.WithString("LanguageCode", LanguageCodeMapper::GetNameForLanguageCode(m_languageCode));
  }

  if(m_volumeKmsKeyIdHasBeenSet)
  {
   payload.WithString("VolumeKmsKeyId", m_volumeKmsKeyId);

  }

  if(m_vpcConfigHasBeenSet)
  {
   payload.WithObject("VpcConfig", m_vpcConfig.Jsonize());

  }

  return payload.View().WriteReadable();
}

Aws::Http::HeaderValueCollection CreateEntityRecognizerRequest::GetRequestSpecificHeaders() const
{
  Aws::Http::HeaderValueCollection headers;
  headers.insert(Aws::Http::HeaderValuePair("X-Amz-Target", "Comprehend_20171127.CreateEntityRecognizer"));
  return headers;

}



