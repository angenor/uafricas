<script setup lang="ts">
/**
 * Coordonnées publiques d'un support média (09p), partagées par les quatre
 * formulaires du back-office : chaîne TV et station radio, en création comme
 * en modification.
 *
 * Un support proposé par un membre arrive avec ses contacts déjà remplis
 * (`ProposerMediaModal`) ; ce bloc permet à l'administrateur de les corriger,
 * et d'en saisir pour les supports créés depuis le back-office.
 */
const email = defineModel<string>('email', { default: '' })
const telephone = defineModel<string>('telephone', { default: '' })
const whatsapp = defineModel<string>('whatsapp', { default: '' })
const siteWeb = defineModel<string>('siteWeb', { default: '' })
const adresse = defineModel<string>('adresse', { default: '' })

const props = withDefaults(defineProps<{
  /** « chaîne » ou « station » : n'apparaît que dans le texte d'aide. */
  libelleSupport?: string
}>(), { libelleSupport: 'support' })
</script>

<template>
  <div class="space-y-4">
    <h3 class="text-lg font-semibold border-b pb-2">Contacts publics</h3>
    <div class="alert alert-info">
      <font-awesome-icon icon="circle-info" />
      <span class="text-sm">
        Affichés sur la page publique de {{ props.libelleSupport }} une fois publié{{ props.libelleSupport === 'la station' ? 'e' : '' }}.
        Tous facultatifs — laisser vide n’affiche rien.
      </span>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <div class="form-control">
        <label class="label"><span class="label-text">E-mail</span></label>
        <input v-model="email" type="email" maxlength="320" class="input input-bordered" placeholder="contact@media.tv">
      </div>
      <div class="form-control">
        <label class="label"><span class="label-text">Téléphone</span></label>
        <input v-model="telephone" type="tel" maxlength="50" class="input input-bordered" placeholder="+225 01 02 03 04 05">
      </div>
      <div class="form-control">
        <label class="label"><span class="label-text">WhatsApp</span></label>
        <input v-model="whatsapp" type="tel" maxlength="50" class="input input-bordered" placeholder="+225 01 02 03 04 05">
      </div>
      <div class="form-control">
        <label class="label"><span class="label-text">Site web</span></label>
        <input v-model="siteWeb" type="text" maxlength="500" class="input input-bordered" placeholder="www.media.tv">
        <label class="label"><span class="label-text-alt">Le préfixe https:// est ajouté s’il manque.</span></label>
      </div>
    </div>

    <div class="form-control">
      <label class="label"><span class="label-text">Adresse</span></label>
      <input v-model="adresse" type="text" maxlength="300" class="input input-bordered" placeholder="Siège, quartier, ville">
    </div>
  </div>
</template>
